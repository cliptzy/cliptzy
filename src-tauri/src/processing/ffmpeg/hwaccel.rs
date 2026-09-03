#[derive(Clone, Debug, PartialEq)]
pub enum HwAccel {
    VideoToolbox, // macOS
    Nvenc,        // NVIDIA
    Amf,          // AMD
    Qsv,          // Intel
    Cpu,          // Fallback
}

impl HwAccel {
    pub fn detect(config_override: Option<&str>) -> Self {
        if let Some(cfg) = config_override {
            match cfg.to_lowercase().as_str() {
                "nvenc" | "nvidia" => return HwAccel::Nvenc,
                "amf" | "amd" => return HwAccel::Amf,
                "qsv" | "intel" => return HwAccel::Qsv,
                "videotoolbox" | "mac" => return HwAccel::VideoToolbox,
                "cpu" | "software" => return HwAccel::Cpu,
                _ => {} // Fallthrough
            }
        }

        #[cfg(target_os = "macos")]
        return HwAccel::VideoToolbox;

        // Default to CPU for maximum compatibility
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
                "-c:v".to_string(),
                "h264_videotoolbox".to_string(),
                "-b:v".to_string(),
                "3000k".to_string(),
            ],
            HwAccel::Nvenc => vec![
                "-c:v".to_string(),
                "h264_nvenc".to_string(),
                "-preset".to_string(),
                "p4".to_string(),
                "-b:v".to_string(),
                "3000k".to_string(),
                "-maxrate".to_string(),
                "4000k".to_string(),
                "-bufsize".to_string(),
                "8000k".to_string(),
            ],
            HwAccel::Amf => vec![
                "-c:v".to_string(),
                "h264_amf".to_string(),
                "-b:v".to_string(),
                "3000k".to_string(),
            ],
            HwAccel::Qsv => vec![
                "-c:v".to_string(),
                "h264_qsv".to_string(),
                "-b:v".to_string(),
                "3000k".to_string(),
            ],
            HwAccel::Cpu => vec![
                "-c:v".to_string(),
                "libx264".to_string(),
                "-preset".to_string(),
                "fast".to_string(),
                "-crf".to_string(),
                "26".to_string(),
                "-maxrate".to_string(),
                "4000k".to_string(),
                "-bufsize".to_string(),
                "8000k".to_string(),
            ],
        }
    }
}
