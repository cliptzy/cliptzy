use rustfft::{num_complex::Complex, FftPlanner};
use std::collections::HashMap;
use std::f32::consts::PI;

pub const WINDOW_SIZE: usize = 4096;

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
pub fn write_wav_segment(path: &std::path::Path, samples: &[f32], sample_rate: u32) -> Result<(), String> {
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

/// Pipeline from raw audio samples to fingerprint hashes.
pub fn process_audio(samples: &[f32]) -> Vec<HashEntry> {
    let spectrogram = get_spectrogram(samples);
    let peaks = get_constellation_map(&spectrogram, 10, 10, 5.0);
    generate_hashes(peaks, 5, 50, 5)
}

/// Find where `short_samples` occurs inside `long_samples` using constellation hashing.
pub fn find_audio_match(
    long_samples: &[f32],
    short_samples: &[f32],
    sample_rate: u32,
) -> Option<AudioMatchResult> {
    if short_samples.len() < WINDOW_SIZE || long_samples.len() < WINDOW_SIZE {
        return None;
    }

    let long_hashes = process_audio(long_samples);
    let short_hashes = process_audio(short_samples);

    if short_hashes.is_empty() || long_hashes.is_empty() {
        return None;
    }

    let min_score = (short_hashes.len() / 5).clamp(5, 10);

    let (frame_offset, score) = find_best_match(&long_hashes, &short_hashes, min_score)?;

    let hop_size = WINDOW_SIZE as f64;
    let start_time_secs = (frame_offset as f64 * hop_size) / sample_rate as f64;

    Some(AudioMatchResult {
        frame_offset,
        score,
        start_time_secs,
    })
}

fn get_spectrogram(samples: &[f32]) -> Vec<Vec<f32>> {
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(WINDOW_SIZE);
    let mut spectrogram = Vec::new();

    let window: Vec<f32> = (0..WINDOW_SIZE)
        .map(|i| 0.5 * (1.0 - (2.0 * PI * i as f32 / (WINDOW_SIZE - 1) as f32).cos()))
        .collect();

    for chunk in samples.chunks_exact(WINDOW_SIZE) {
        let mut buffer: Vec<Complex<f32>> = chunk
            .iter()
            .enumerate()
            .map(|(i, &val)| Complex {
                re: val * window[i],
                im: 0.0,
            })
            .collect();

        fft.process(&mut buffer);

        let magnitudes: Vec<f32> = buffer
            .iter()
            .take(WINDOW_SIZE / 2)
            .map(|c| c.norm())
            .collect();

        spectrogram.push(magnitudes);
    }

    spectrogram
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
            'neighbor: for nt in t_start..t_end {
                for nf in f_start..f_end {
                    if nt == t && nf == f {
                        continue;
                    }
                    if spectrogram[nt][nf] >= current_mag {
                        is_local_max = false;
                        break 'neighbor;
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

fn find_best_match(
    long_hashes: &[HashEntry],
    short_hashes: &[HashEntry],
    min_score: usize,
) -> Option<(isize, usize)> {
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

    histogram
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .filter(|&(_, count)| count >= min_score)
}
