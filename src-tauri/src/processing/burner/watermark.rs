use crate::processing::ffmpeg::filters::{FilterGraph, FilterNode};

pub fn apply_watermark(
    graph: &mut FilterGraph,
    input_v: &str,
    input_idx: &mut usize,
    position: &str,
) -> String {
    let wm_input = format!("{}:v", input_idx);
    
    let (x, y) = match position {
        "top" => ("(W-w)/2", "30"),
        "bottom" => ("(W-w)/2", "H-h-30"),
        _ => ("(W-w)/2", "(H-h)/2"),
    };

    let overlay = FilterNode::new("overlay")
        .param("x", x)
        .param("y", y)
        .inputs(&[input_v, &wm_input])
        .outputs(&["v_wm"]);

    graph.add_node(overlay);
    *input_idx += 1;
    "v_wm".to_string()
}
