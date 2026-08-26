use crate::processing::ffmpeg::filters::{FilterGraph, FilterNode};

pub fn apply_vfx(graph: &mut FilterGraph, input_v: &str, input_idx: &mut usize) -> String {
    let vfx_input = format!("{}:v", input_idx);
    let chromakey = FilterNode::new("chromakey")
        .param("color", "0x00FF00")
        .param("similarity", "0.2")
        .inputs(&[&vfx_input])
        .outputs(&["vfx_keyed"]);

    let overlay = FilterNode::new("overlay")
        .param("x", "(W-w)/2")
        .param("y", "(H-h)/2")
        .param("eof_action", "pass")
        .inputs(&[input_v, "vfx_keyed"])
        .outputs(&["v_vfx"]);

    graph.add_node(chromakey);
    graph.add_node(overlay);
    *input_idx += 1;
    "v_vfx".to_string()
}
