use crate::orchestrator::job_cache::sanitize_cache_token;
use std::path::{Path, PathBuf};

/// Direktori cache global untuk aset restreamer (dibagi antar job / match).
pub fn restreamer_cache_root() -> PathBuf {
    crate::paths::app_data_dir()
        .join("cache")
        .join("restreamers")
}

pub fn restreamer_cache_dir(restreamer_id: &str) -> PathBuf {
    restreamer_cache_root().join(sanitize_cache_token(restreamer_id))
}

pub fn restreamer_audio_m4a(restreamer_id: &str) -> PathBuf {
    restreamer_cache_dir(restreamer_id).join("audio.m4a")
}

pub fn restreamer_audio_wav(restreamer_id: &str) -> PathBuf {
    restreamer_cache_dir(restreamer_id).join("audio_16k.wav")
}

pub fn restreamer_fingerprint_bin(restreamer_id: &str) -> PathBuf {
    restreamer_cache_dir(restreamer_id).join("fingerprint.bin")
}

/// Cache hasil sinkronisasi per kombinasi video utama + momen epik.
pub fn restreamer_sync_cache_path(
    restreamer_id: &str,
    main_video_id: &str,
    moments_hash: &str,
) -> PathBuf {
    restreamer_cache_dir(restreamer_id)
        .join("sync")
        .join(format!(
            "{}_{}.json",
            sanitize_cache_token(main_video_id),
            moments_hash
        ))
}

/// Salin aset restreamer lama dari job dir ke cache global bila belum ada.
pub fn migrate_legacy_job_assets(job_dir: &Path, restreamer_id: &str) {
    let cache_dir = restreamer_cache_dir(restreamer_id);
    if let Err(e) = std::fs::create_dir_all(&cache_dir) {
        log::warn!(
            "Gagal membuat direktori cache restreamer {:?}: {}",
            cache_dir,
            e
        );
        return;
    }

    let token = sanitize_cache_token(restreamer_id);
    let legacy_m4a = job_dir.join(format!("restr_{}.m4a", token));
    let legacy_wav = job_dir.join(format!("restr_{}_16k.wav", token));
    let legacy_fp = crate::orchestrator::job_cache::cache_file(
        job_dir,
        &format!("restr_{}_fingerprint.bin", token),
    );

    let global_m4a = restreamer_audio_m4a(restreamer_id);
    let global_wav = restreamer_audio_wav(restreamer_id);
    let global_fp = restreamer_fingerprint_bin(restreamer_id);

    copy_if_missing(&legacy_m4a, &global_m4a);
    copy_if_missing(&legacy_wav, &global_wav);
    copy_if_missing(&legacy_fp, &global_fp);
}

fn copy_if_missing(from: &Path, to: &Path) {
    if to.exists() || !from.exists() {
        return;
    }
    if let Some(parent) = to.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    match std::fs::copy(from, to) {
        Ok(_) => log::info!("Migrasi cache restreamer: {:?} -> {:?}", from, to),
        Err(e) => log::warn!("Gagal migrasi {:?} ke {:?}: {}", from, to, e),
    }
}
