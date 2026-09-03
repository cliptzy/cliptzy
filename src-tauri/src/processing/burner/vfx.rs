use crate::processing::ffmpeg::filters::{FilterGraph, FilterNode};

pub fn apply_vfx(
    graph: &mut FilterGraph,
    input_v: &str,
    input_a: &str,
    input_idx: &mut usize,
    start_time: f64,
    end_time: f64,
) -> (String, String) {
    let vfx_keyed_name = format!("vfx_keyed_{}", input_idx);
    let vfx_input = format!("{}:v", input_idx);
    let vfx_delayed_name = format!("vfx_delayed_{}", input_idx);

    let setpts = FilterNode::new("setpts")
        .param("expr", &format!("PTS-STARTPTS+{}/TB", start_time))
        .inputs(&[vfx_input.as_str()])
        .outputs(&[vfx_delayed_name.as_str()]);

    let chromakey = FilterNode::new("chromakey")
        .param("color", "0x00FF00")
        .param("similarity", "0.2")
        .inputs(&[vfx_delayed_name.as_str()])
        .outputs(&[vfx_keyed_name.as_str()]);

    let output_name = format!("v_vfx_{}", input_idx);
    let overlay = FilterNode::new("overlay")
        .param("x", "(W-w)/2")
        .param("y", "(H-h)/2")
        .param("eof_action", "pass")
        .param("enable", &format!("between(t,{},{})", start_time, end_time))
        .inputs(&[input_v, vfx_keyed_name.as_str()])
        .outputs(&[output_name.as_str()]);

    graph.add_node(setpts);
    graph.add_node(chromakey);
    graph.add_node(overlay);

    let a_input = format!("{}:a", input_idx);
    let a_delayed = format!("a_vfx_delayed_{}", input_idx);
    let start_ms = (start_time * 1000.0).round() as u64;
    let adelay = FilterNode::new("adelay")
        .param("delays", &format!("{}|{}", start_ms, start_ms))
        .inputs(&[a_input.as_str()])
        .outputs(&[a_delayed.as_str()]);

    let a_mixed = format!("a_vfx_mixed_{}", input_idx);
    let amix = FilterNode::new("amix")
        .param("inputs", "2")
        .param("duration", "first")
        .param("dropout_transition", "0")
        .inputs(&[input_a, a_delayed.as_str()])
        .outputs(&[a_mixed.as_str()]);

    graph.add_node(adelay);
    graph.add_node(amix);

    *input_idx += 1;
    (output_name, a_mixed)
}
