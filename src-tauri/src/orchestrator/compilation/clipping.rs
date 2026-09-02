use super::models::RestreamerClip;
use crate::error::CliptzyError;
use crate::orchestrator::pipeline::PipelineContext;

pub async fn clip_and_label_restreamers(
    ctx: &PipelineContext,
    clips: Vec<RestreamerClip>,
) -> Result<Vec<String>, CliptzyError> {
    log::info!("Memulai Clipping & Labeling Restreamer (Phase 6)");

    let cookies_path = ctx.config.browser.as_deref().map(|s| s.to_string());
    let job_dir = &ctx.job_dir;
    std::fs::create_dir_all(job_dir)?;

    let app_dir = crate::paths::app_data_dir();
    let font_path = app_dir.join("assets").join("Geist-Black.ttf");
    let font_path_str = font_path
        .to_string_lossy()
        .to_string()
        .replace('\\', "/")
        .replace(":", "\\:");

    let mut output_paths = Vec::new();
    let hwaccel = crate::processing::ffmpeg::hwaccel::HwAccel::detect(Some(&ctx.config.hw_accel));
    let encode_args = hwaccel.encode_args();

    for (i, clip) in clips.iter().enumerate() {
        log::info!(
            "Memproses klip {} ({} hingga {})",
            clip.description,
            clip.start,
            clip.end
        );

        let output_mp4 = job_dir.join(format!("restr_clip_{}.mp4", i));

        if output_mp4.exists() {
            log::info!("Klip {} sudah tersedia di cache", i);
            output_paths.push(output_mp4.to_string_lossy().to_string());
            continue;
        }

        let raw_mp4 = job_dir.join(format!("raw_restr_clip_{}.mp4", i));

        if !raw_mp4.exists() {
            log::info!("Mengunduh segmen mentah untuk klip {}...", i);
            let mut ytdlp_cmd = tokio::process::Command::new(&ctx.deps.ytdlp);
            ytdlp_cmd
                .arg("--download-sections")
                .arg(format!("*{}-{}", clip.start, clip.end))
                .arg("-f")
                .arg("bestvideo[height<=1080][ext=mp4]+bestaudio[ext=m4a]/best")
                .arg("-o")
                .arg(raw_mp4.to_string_lossy().to_string())
                .arg("--extractor-args")
                .arg("youtube:player-client=android,web,default")
                .arg("--remote-components")
                .arg("ejs:github");

            if let Some(browser) = &cookies_path {
                if !browser.is_empty() {
                    ytdlp_cmd.arg("--cookies-from-browser").arg(browser);
                }
            }
            ytdlp_cmd.arg(&clip.restreamer_url);

            let mut stage = crate::processing::ffmpeg::runner::PipelineStage::new(
                "yt-dlp Download Section",
                ytdlp_cmd,
            );

            if let Err(e) = stage.execute(ctx.cancel_token.clone()).await {
                log::error!(
                    "[Compilation] Gagal mendownload segmen mentah klip {}: {}",
                    i,
                    e
                );
                continue;
            }
        }

        let channel_name = clip
            .restreamer_url
            .split('@')
            .nth(1)
            .unwrap_or("Restreamer");

        use crate::processing::ffmpeg::filters::{FilterGraph, FilterNode};
        let skip_crop = ctx.config.compilation.crop_mode == "none";

        let mut drawtext_node = FilterNode::new("drawtext")
            .param("text", &format!("'{}'", channel_name))
            .param("fontcolor", "white")
            .param("fontsize", "48")
            .param("box", "1")
            .param("boxcolor", "black@0.5")
            .param("boxborderw", "10")
            .param("x", "50")
            .param("y", "50");

        if font_path.exists() {
            drawtext_node = drawtext_node.param("fontfile", &format!("'{}'", font_path_str));
        }

        let filter_str = if skip_crop {
            log::info!(
                "Mode tanpa crop: mempertahankan resolusi asli untuk klip {}",
                i
            );
            let mut graph = FilterGraph::new();
            graph.add_node(drawtext_node);
            graph.to_string()
        } else {
            let mut graph = FilterGraph::new();
            let scale_node = FilterNode::new("scale")
                .param("", "1920:1080")
                .param("force_original_aspect_ratio", "decrease");
            let pad_node = FilterNode::new("pad").param("", "1920:1080:(ow-iw)/2:(oh-ih)/2");
            graph.add_node(scale_node.outputs(&["scaled"]));
            graph.add_node(pad_node.inputs(&["scaled"]).outputs(&["padded"]));
            graph.add_node(drawtext_node.inputs(&["padded"]));
            graph.to_string()
        };

        let mut ff_cmd = tokio::process::Command::new(&ctx.deps.ffmpeg);
        ff_cmd.arg("-i").arg(raw_mp4.to_string_lossy().to_string());

        ff_cmd.arg("-vf").arg(&filter_str);
        for arg in encode_args.iter() {
            ff_cmd.arg(arg);
        }
        ff_cmd
            .arg("-pix_fmt").arg("yuv420p")
            .arg("-movflags").arg("+faststart")
            .arg("-c:a").arg("aac")
            .arg("-b:a").arg("192k");
        ff_cmd.arg("-y");
        ff_cmd.arg(output_mp4.to_string_lossy().to_string());

        log::info!("Spawn FFmpeg clip untuk {:?}", clip.description);

        let mut stage =
            crate::processing::ffmpeg::runner::PipelineStage::new("Clip & Label", ff_cmd);

        match stage.execute(ctx.cancel_token.clone()).await {
            Ok(()) => {
                output_paths.push(output_mp4.to_string_lossy().to_string());
            }
            Err(e) => {
                log::error!("[Compilation] FFmpeg gagal memotong klip {}: {}", i, e);
            }
        }
    }

    Ok(output_paths)
}
