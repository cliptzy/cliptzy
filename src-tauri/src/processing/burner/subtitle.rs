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

    let sub_node = FilterNode::new("subtitles")
        .param("", &final_ass)
        .inputs(&[input_v])
        .outputs(&["v_subbed"]);

    graph.add_node(sub_node);
    "v_subbed".to_string()
}
