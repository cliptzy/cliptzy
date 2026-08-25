pub enum HwAccel {
    VideoToolbox, // macOS
    Nvenc,        // NVIDIA
    Amf,          // AMD
    Qsv,          // Intel
    Cpu,          // Fallback
}

impl HwAccel {
    pub fn detect(_config_override: Option<&str>) -> Self {
        // Nanti implementasi deteksi hardware sungguhan 
        // dengan menggunakan eksekusi subprocess `ffmpeg -hwaccels` atau deteksi OS
        
        #[cfg(target_os = "macos")]
        return HwAccel::VideoToolbox;

        #[cfg(target_os = "windows")]
        return HwAccel::Nvenc; // Simplified fallback, actually we should detect

        #[cfg(target_os = "linux")]
        return HwAccel::Cpu; 

        #[allow(unreachable_code)]
        HwAccel::Cpu
    }

    pub fn encoder(&self) -> &str {
        match self {
            HwAccel::VideoToolbox => "h264_videotoolbox",
            HwAccel::Nvenc => "h264_nvenc",
            HwAccel::Amf => "h264_amf",
            HwAccel::Qsv => "h264_qsv",
            HwAccel::Cpu => "libx264",
        }
    }

    pub fn encode_args(&self) -> Vec<String> {
        match self {
            HwAccel::VideoToolbox => vec![
                "-c:v".to_string(), "h264_videotoolbox".to_string(),
                "-b:v".to_string(), "5000k".to_string(),
            ],
            HwAccel::Nvenc => vec![
                "-c:v".to_string(), "h264_nvenc".to_string(),
                "-preset".to_string(), "p4".to_string(),
            ],
            HwAccel::Amf => vec![
                "-c:v".to_string(), "h264_amf".to_string(),
            ],
            HwAccel::Qsv => vec![
                "-c:v".to_string(), "h264_qsv".to_string(),
            ],
            HwAccel::Cpu => vec![
                "-c:v".to_string(), "libx264".to_string(),
                "-preset".to_string(), "fast".to_string(),
            ],
        }
    }
}
