use crate::processing::ffmpeg::filters::{FilterGraph, FilterNode};
use crate::transcription::models::SubtitleConfig;

pub fn apply_subtitle(
    graph: &mut FilterGraph,
    input_v: &str,
    ass_path: &str,
    config: &Option<SubtitleConfig>,
) -> String {
    let safe_path = ass_path.replace("\\", "/");
    let escaped_ass = safe_path.replace(":", "\\:");
    let final_ass = format!("'{}'", escaped_ass);
    
    let mut sub_node = FilterNode::new("subtitles")
        .param("", &final_ass)
        .inputs(&[input_v])
        .outputs(&["v_subbed"]);

    if let Some(cfg) = config {
        let mut force_style = Vec::new();
        
        if !cfg.font.is_empty() {
            force_style.push(format!("Fontname={}", cfg.font));
        }
        if cfg.font_size > 0 {
            force_style.push(format!("Fontsize={}", cfg.font_size));
        }
        if !cfg.primary_color.is_empty() {
            force_style.push(format!("PrimaryColour={}", cfg.primary_color));
        }
        if !cfg.back_color.is_empty() {
            force_style.push(format!("BackColour={}", cfg.back_color));
        }
        if cfg.border_style > 0 {
            force_style.push(format!("BorderStyle={}", cfg.border_style));
        }
        if cfg.alignment > 0 {
            force_style.push(format!("Alignment={}", cfg.alignment));
        }
        if !cfg.outline_color.is_empty() {
            force_style.push(format!("OutlineColour={}", cfg.outline_color));
        }
        force_style.push(format!("Outline={}", cfg.outline));
        force_style.push(format!("Shadow={}", cfg.shadow));
        force_style.push(format!("MarginV={}", cfg.margin_v));
        
        if !force_style.is_empty() {
            let style_str = force_style.join(",");
            sub_node = sub_node.param("force_style", &format!("'{}'", style_str));
        }
    }

    graph.add_node(sub_node);
    "v_subbed".to_string()
}
