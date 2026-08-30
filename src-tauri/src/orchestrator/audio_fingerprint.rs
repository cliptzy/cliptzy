use rustfft::{num_complex::Complex, FftPlanner};
use std::collections::HashMap;
use std::f32::consts::PI;

pub const WINDOW_SIZE: usize = 4096;
pub const HOP_SIZE: usize = WINDOW_SIZE / 2;

/// Cached fingerprint index for a long audio track (e.g. restreamer VOD).
pub struct AudioFingerprintDb {
    pub hashes: Vec<HashEntry>,
    pub preprocessed: Vec<f32>,
}

#[derive(Debug, Clone, Copy)]
struct Peak {
    time_idx: usize,
    freq_idx: usize,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct AudioHash {
    freq_anchor: usize,
    freq_target: usize,
    delta_time: usize,
}

#[derive(Debug, Clone)]
pub struct HashEntry {
    hash: AudioHash,
    anchor_time: usize,
}

#[derive(Debug, Clone)]
pub struct AudioMatchResult {
    pub frame_offset: isize,
    pub score: usize,
    pub start_time_secs: f64,
    pub method: String,
}

/// Decode mono f32 samples from a 16-bit PCM WAV file.
pub fn decode_wav(path: &str) -> Result<(Vec<f32>, u32), String> {
    let mut reader = hound::WavReader::open(path).map_err(|e| e.to_string())?;
    let sample_rate = reader.spec().sample_rate;

    let samples: Vec<f32> = reader
        .samples::<i16>()
        .map(|s| s.unwrap_or(0) as f32 / 32768.0)
        .collect();

    Ok((samples, sample_rate))
}

/// Write mono f32 samples to a 16-bit PCM WAV file.
pub fn write_wav_segment(
    path: &std::path::Path,
    samples: &[f32],
    sample_rate: u32,
) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(path, spec).map_err(|e| e.to_string())?;
    for &sample in samples {
        let amplitude = (sample * 32767.0).clamp(-32768.0, 32767.0) as i16;
        writer.write_sample(amplitude).map_err(|e| e.to_string())?;
    }
    writer.finalize().map_err(|e| e.to_string())?;
    Ok(())
}

/// Build a reusable fingerprint database from long audio (call once per restreamer VOD).
pub fn build_fingerprint_db(samples: &[f32]) -> AudioFingerprintDb {
    let preprocessed = preprocess_for_matching(samples);
    let hashes = process_audio_internal(&preprocessed);
    AudioFingerprintDb {
        hashes,
        preprocessed,
    }
}

/// Find where `short_samples` occurs inside pre-built long audio database.
pub fn find_match_in_db(
    db: &AudioFingerprintDb,
    short_samples: &[f32],
    sample_rate: u32,
) -> Option<AudioMatchResult> {
    if short_samples.len() < WINDOW_SIZE || db.preprocessed.len() < WINDOW_SIZE {
        return None;
    }

    let short_preprocessed = preprocess_for_matching(short_samples);

    let fingerprint_match = find_fingerprint_match(&db.hashes, &short_preprocessed);
    let envelope_match = find_envelope_match(&db.preprocessed, &short_preprocessed, sample_rate);

    pick_best_match(fingerprint_match, envelope_match)
}

/// Find where `short_samples` occurs inside `long_samples` (convenience wrapper).
pub fn find_audio_match(
    long_samples: &[f32],
    short_samples: &[f32],
    sample_rate: u32,
) -> Option<AudioMatchResult> {
    let db = build_fingerprint_db(long_samples);
    find_match_in_db(&db, short_samples, sample_rate)
}

fn pick_best_match(
    fingerprint: Option<AudioMatchResult>,
    envelope: Option<AudioMatchResult>,
) -> Option<AudioMatchResult> {
    match (fingerprint, envelope) {
        (Some(fp), Some(env)) => {
            if fp.score >= 4 && fp.score as f32 >= env.score as f32 * 0.6 {
                Some(fp)
            } else if env.score >= 3 {
                Some(env)
            } else {
                Some(fp)
            }
        }
        (Some(fp), None) => Some(fp),
        (None, Some(env)) => Some(env),
        (None, None) => None,
    }
}

fn find_fingerprint_match(long_hashes: &[HashEntry], short_samples: &[f32]) -> Option<AudioMatchResult> {
    let short_hashes = process_audio_internal(short_samples);
    if short_hashes.is_empty() || long_hashes.is_empty() {
        return None;
    }

    let min_score = (short_hashes.len() / 8).clamp(3, 8);
    let (frame_offset, score, confidence) =
        find_best_match_with_confidence(long_hashes, &short_hashes, min_score)?;

    if confidence < 1.2 && score < min_score + 2 {
        return None;
    }

    Some(AudioMatchResult {
        frame_offset,
        score,
        start_time_secs: frame_to_seconds(frame_offset),
        method: format!("fingerprint(conf={:.2})", confidence),
    })
}

fn find_envelope_match(
    long_samples: &[f32],
    short_samples: &[f32],
    sample_rate: u32,
) -> Option<AudioMatchResult> {
    const CHUNK_SAMPLES: usize = 800; // 50ms at 16kHz — onset/transient focused
    const MIN_CORRELATION: f32 = 0.32;

    let long_env = onset_envelope(long_samples, CHUNK_SAMPLES);
    let short_env = onset_envelope(short_samples, CHUNK_SAMPLES);

    if short_env.len() < 8 || long_env.len() <= short_env.len() {
        return None;
    }

    let short_mean = short_env.iter().sum::<f32>() / short_env.len() as f32;
    let short_std = (short_env
        .iter()
        .map(|x| (x - short_mean).powi(2))
        .sum::<f32>()
        / short_env.len() as f32)
        .sqrt();
    if short_std < 1e-6 {
        return None;
    }
    let short_norm: Vec<f32> = short_env
        .iter()
        .map(|x| (x - short_mean) / short_std)
        .collect();

    let search_range = long_env.len() - short_env.len();
    let mut max_corr = -1.0_f32;
    let mut best_offset = 0usize;
    let mut second_best = -1.0_f32;

    for offset in 0..=search_range {
        let slice = &long_env[offset..offset + short_env.len()];
        let slice_mean = slice.iter().sum::<f32>() / slice.len() as f32;
        let slice_std = (slice.iter().map(|x| (x - slice_mean).powi(2)).sum::<f32>()
            / slice.len() as f32)
            .sqrt();
        if slice_std < 1e-6 {
            continue;
        }

        let mut corr = 0.0_f32;
        for i in 0..short_norm.len() {
            corr += short_norm[i] * ((slice[i] - slice_mean) / slice_std);
        }
        corr /= short_norm.len() as f32;

        if corr > max_corr {
            second_best = max_corr;
            max_corr = corr;
            best_offset = offset;
        } else if corr > second_best {
            second_best = corr;
        }
    }

    if max_corr < MIN_CORRELATION {
        return None;
    }

    if second_best > 0.0 && max_corr / second_best < 1.08 {
        return None;
    }

    let frame_offset = ((best_offset * CHUNK_SAMPLES) / HOP_SIZE) as isize;
    let start_time_secs = (best_offset * CHUNK_SAMPLES) as f64 / sample_rate as f64;
    let score = (max_corr * 100.0) as usize;

    Some(AudioMatchResult {
        frame_offset,
        score,
        start_time_secs,
        method: format!("envelope(corr={:.3})", max_corr),
    })
}

fn frame_to_seconds(frame_offset: isize) -> f64 {
    (frame_offset as f64 * HOP_SIZE as f64) / 16_000.0
}

/// Band-pass + normalize to emphasize game SFX over dominant voice.
fn preprocess_for_matching(samples: &[f32]) -> Vec<f32> {
    let hp = biquad_filter(samples, 16_000.0, 280.0, true);
    let bp = biquad_filter(&hp, 16_000.0, 5_500.0, false);
    normalize_rms(&bp, 0.15)
}

fn normalize_rms(samples: &[f32], target_rms: f32) -> Vec<f32> {
    let rms = (samples.iter().map(|s| s * s).sum::<f32>() / samples.len().max(1) as f32).sqrt();
    if rms < 1e-8 {
        return samples.to_vec();
    }
    let gain = target_rms / rms;
    samples.iter().map(|s| (s * gain).clamp(-1.0, 1.0)).collect()
}

struct Biquad {
    b0: f32,
    b1: f32,
    b2: f32,
    a1: f32,
    a2: f32,
    z1: f32,
    z2: f32,
}

impl Biquad {
    fn new(sample_rate: f32, freq: f32, q: f32, highpass: bool) -> Self {
        let omega = 2.0 * PI * freq / sample_rate;
        let sin = omega.sin();
        let cos = omega.cos();
        let alpha = sin / (2.0 * q);

        let (b0, b1, b2, a0, a1, a2) = if highpass {
            let b0 = (1.0 + cos) / 2.0;
            let b1 = -(1.0 + cos);
            let b2 = (1.0 + cos) / 2.0;
            let a0 = 1.0 + alpha;
            let a1 = -2.0 * cos;
            let a2 = 1.0 - alpha;
            (b0, b1, b2, a0, a1, a2)
        } else {
            let b0 = (1.0 - cos) / 2.0;
            let b1 = 1.0 - cos;
            let b2 = (1.0 - cos) / 2.0;
            let a0 = 1.0 + alpha;
            let a1 = -2.0 * cos;
            let a2 = 1.0 - alpha;
            (b0, b1, b2, a0, a1, a2)
        };

        Self {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0,
            a2: a2 / a0,
            z1: 0.0,
            z2: 0.0,
        }
    }

    fn process(&mut self, x: f32) -> f32 {
        let y = self.b0 * x + self.z1;
        self.z1 = self.b1 * x - self.a1 * y + self.z2;
        self.z2 = self.b2 * x - self.a2 * y;
        y
    }
}

fn biquad_filter(samples: &[f32], sample_rate: f32, freq: f32, highpass: bool) -> Vec<f32> {
    let mut filter = Biquad::new(sample_rate, freq, 0.707, highpass);
    samples.iter().map(|&s| filter.process(s)).collect()
}

fn onset_envelope(samples: &[f32], chunk_size: usize) -> Vec<f32> {
    let mut flux = Vec::new();
    let mut prev = 0.0_f32;
    for &s in samples {
        let rectified = s.abs();
        flux.push((rectified - prev).max(0.0));
        prev = rectified;
    }

    flux.chunks(chunk_size)
        .map(|c| c.iter().sum::<f32>() / c.len() as f32)
        .collect()
}

fn process_audio_internal(samples: &[f32]) -> Vec<HashEntry> {
    let spectrogram = get_spectrogram(samples);
    let threshold = adaptive_peak_threshold(&spectrogram);
    let peaks = get_constellation_map(&spectrogram, 8, 8, threshold);
    generate_hashes(peaks, 5, 50, 6)
}

fn adaptive_peak_threshold(spectrogram: &[Vec<f32>]) -> f32 {
    let mut values: Vec<f32> = spectrogram
        .iter()
        .flat_map(|frame| frame.iter().copied())
        .filter(|&v| v > 0.0)
        .collect();
    if values.is_empty() {
        return 1.5;
    }
    values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let idx = (values.len() as f32 * 0.88) as usize;
    let p88 = values[idx.min(values.len() - 1)];
    p88.max(1.2)
}

fn get_spectrogram(samples: &[f32]) -> Vec<Vec<f32>> {
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(WINDOW_SIZE);
    let mut spectrogram = Vec::new();

    let window: Vec<f32> = (0..WINDOW_SIZE)
        .map(|i| 0.5 * (1.0 - (2.0 * PI * i as f32 / (WINDOW_SIZE - 1) as f32).cos()))
        .collect();

    // Game SFX band ~300Hz–5.5kHz at 16kHz sample rate
    let min_bin = (300.0 * WINDOW_SIZE as f32 / 16_000.0) as usize;
    let max_bin = (5_500.0 * WINDOW_SIZE as f32 / 16_000.0) as usize;

    let mut offset = 0;
    while offset + WINDOW_SIZE <= samples.len() {
        let chunk = &samples[offset..offset + WINDOW_SIZE];
        let mut buffer: Vec<Complex<f32>> = chunk
            .iter()
            .enumerate()
            .map(|(i, &val)| Complex {
                re: val * window[i],
                im: 0.0,
            })
            .collect();

        fft.process(&mut buffer);

        let mut magnitudes: Vec<f32> = buffer
            .iter()
            .take(WINDOW_SIZE / 2)
            .map(|c| c.norm())
            .collect();

        for (i, mag) in magnitudes.iter_mut().enumerate() {
            if i < min_bin || i > max_bin {
                *mag = 0.0;
            }
        }

        whiten_frame(&mut magnitudes);
        emphasize_percussive(&mut magnitudes);
        spectrogram.push(magnitudes);
        offset += HOP_SIZE;
    }

    spectrogram
}

fn whiten_frame(magnitudes: &mut [f32]) {
    let sum: f32 = magnitudes.iter().sum();
    let mean = sum / magnitudes.len().max(1) as f32;
    for mag in magnitudes.iter_mut() {
        if *mag > 0.0 {
            *mag = (*mag / (mean + 1e-6)).ln_1p();
        }
    }
}

fn emphasize_percussive(magnitudes: &mut [f32]) {
    let median: f32 = {
        let mut vals: Vec<f32> = magnitudes.iter().copied().filter(|&v| v > 0.0).collect();
        if vals.is_empty() {
            return;
        }
        vals.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        vals[vals.len() / 2]
    };
    for mag in magnitudes.iter_mut() {
        if *mag > 0.0 {
            *mag = (*mag - median * 0.5).max(0.0);
        }
    }
}

fn get_constellation_map(
    spectrogram: &[Vec<f32>],
    neighborhood_t: usize,
    neighborhood_f: usize,
    threshold: f32,
) -> Vec<Peak> {
    let mut peaks = Vec::new();
    let time_frames = spectrogram.len();
    if time_frames == 0 {
        return peaks;
    }
    let freq_bins = spectrogram[0].len();

    for t in 0..time_frames {
        for f in 0..freq_bins {
            let current_mag = spectrogram[t][f];
            if current_mag < threshold {
                continue;
            }

            let t_start = t.saturating_sub(neighborhood_t);
            let t_end = (t + neighborhood_t + 1).min(time_frames);
            let f_start = f.saturating_sub(neighborhood_f);
            let f_end = (f + neighborhood_f + 1).min(freq_bins);

            let mut is_local_max = true;
            for nt in t_start..t_end {
                if !is_local_max {
                    break;
                }
                for nf in f_start..f_end {
                    if nt == t && nf == f {
                        continue;
                    }
                    if spectrogram[nt][nf] >= current_mag {
                        is_local_max = false;
                        break;
                    }
                }
            }

            if is_local_max {
                peaks.push(Peak {
                    time_idx: t,
                    freq_idx: f,
                });
            }
        }
    }
    peaks
}

fn generate_hashes(
    mut peaks: Vec<Peak>,
    target_zone_start: usize,
    target_zone_end: usize,
    fan_out: usize,
) -> Vec<HashEntry> {
    peaks.sort_by(|a, b| a.time_idx.cmp(&b.time_idx));
    let mut hashes = Vec::new();
    let total = peaks.len();

    for i in 0..total {
        let anchor = &peaks[i];
        let mut found = 0;

        for j in (i + 1)..total {
            let target = &peaks[j];
            let dt = target.time_idx.saturating_sub(anchor.time_idx);

            if dt < target_zone_start {
                continue;
            }
            if dt > target_zone_end || found >= fan_out {
                break;
            }

            hashes.push(HashEntry {
                hash: AudioHash {
                    freq_anchor: anchor.freq_idx,
                    freq_target: target.freq_idx,
                    delta_time: dt,
                },
                anchor_time: anchor.time_idx,
            });
            found += 1;
        }
    }
    hashes
}

fn find_best_match_with_confidence(
    long_hashes: &[HashEntry],
    short_hashes: &[HashEntry],
    min_score: usize,
) -> Option<(isize, usize, f32)> {
    let mut db: HashMap<AudioHash, Vec<usize>> = HashMap::new();
    for entry in long_hashes {
        db.entry(entry.hash).or_default().push(entry.anchor_time);
    }

    let mut histogram: HashMap<isize, usize> = HashMap::new();
    for query in short_hashes {
        if let Some(db_times) = db.get(&query.hash) {
            for &db_t in db_times {
                let offset = (db_t as isize) - (query.anchor_time as isize);
                *histogram.entry(offset).or_insert(0) += 1;
            }
        }
    }

    let mut ranked: Vec<(isize, usize)> = histogram.into_iter().collect();
    ranked.sort_by(|a, b| b.1.cmp(&a.1));

    let (best_offset, best_score) = ranked.first()?;
    if *best_score < min_score {
        return None;
    }

    let second_score = ranked.get(1).map(|(_, s)| *s).unwrap_or(0);
    let confidence = if second_score > 0 {
        *best_score as f32 / second_score as f32
    } else {
        *best_score as f32
    };

    Some((*best_offset, *best_score, confidence))
}
