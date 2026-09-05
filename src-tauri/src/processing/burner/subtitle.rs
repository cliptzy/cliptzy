use crate::processing::ffmpeg::filters::{FilterGraph, FilterNode};
use crate::transcription::models::SubtitleConfig;

pub fn apply_subtitle(
    graph: &mut FilterGraph,
    input_v: &str,
    ass_path: &str,
    _config: &Option<SubtitleConfig>,
) -> String {
    let safe_path = ass_path.replace("\\", "/");
    let escaped_ass = safe_path.replace(":", "\\:");
    let final_ass = format!("'{}'", escaped_ass);

    let output_name = format!("v_subbed_{}", graph.nodes.len());
    let sub_node = FilterNode::new("subtitles")
        .param("", &final_ass)
        .inputs(&[input_v])
        .outputs(&[&output_name]);

    graph.add_node(sub_node);
    output_name
}
