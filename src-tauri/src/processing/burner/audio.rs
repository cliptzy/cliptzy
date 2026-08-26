use crate::processing::ffmpeg::filters::{FilterGraph, FilterNode};

pub fn apply_normalization(graph: &mut FilterGraph) -> String {
    let loudnorm = FilterNode::new("loudnorm")
        .param("I", "-16")
        .param("LRA", "11")
        .param("TP", "-1.5")
        .inputs(&["0:a"])
        .outputs(&["a_norm"]);

    graph.add_node(loudnorm);
    "a_norm".to_string()
}
