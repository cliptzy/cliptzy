use crate::error::CliptzyError;
use std::path::Path;

// Nanti implementasi wrapper ffprobe dan ffmpeg copy
pub async fn probe_local_video(_path: &Path) -> Result<(), CliptzyError> {
    // Placeholder untuk Phase 1
    Ok(())
}

pub async fn cut_local_segment(
    _input_path: &Path,
    _start: f64,
    _end: f64,
    _output_path: &Path,
) -> Result<(), CliptzyError> {
    // Placeholder untuk Phase 1
    // ffmpeg -ss start -to end -i input -c copy output
    Ok(())
}
