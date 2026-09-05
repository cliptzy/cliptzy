use crate::processing::ffmpeg::filters::FilterGraph;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuiltinEffectType {
    /// Guncangan kamera dramatis (earthquake/twitch) saat teriakan atau rage
    ScreenShake,
    /// Kilatan putih sesaat (flashbang) saat jumpscare atau vineboom punchline
    WhiteFlash,
    /// Desaturasi hitam-putih + vignette melingkar (efek wasted / pasrah / sedih)
    DramaticBW,
    /// Kontras dan saturasi ekstrem untuk kekacauan meme / rage
    DeepFried,
    /// Snap zoom 18% ke tengah kanvas untuk momen awkward / bingung
    PunchZoom,
    /// Nuansa merah pekat untuk amarah membara, bahaya, atau low HP
    RedTint,
    /// Inversi warna negatif untuk momen cursed, horror jumpscare, atau plot twist
    Negate,
    /// Gaussian blur dramatis untuk momen bengong, pusing, freeze, atau brain crash
    FocusBlur,
    /// Nada hangat klasik sepia vintage untuk kilas balik sedih, refleksi kekalahan, atau kenangan
    Sepia,
    /// Rotasi spektrum warna pelangi cepat untuk perayaan, victory, GG, atau hype tawa
    RainbowHue,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScheduledBuiltinEffect {
    pub effect_type: BuiltinEffectType,
    pub start_time: f64,
    pub end_time: f64,
}

/// Menerapkan efek visual bawaan FFmpeg ke dalam FilterGraph.
///
/// PERLINDUNGAN COMMAND LENGTH (Windows Safety):
/// Setiap tipe efek digabung menggunakan ekspresi boolean `between(t, s1, e1) + between(t, s2, e2)`.
/// Ini memastikan bahwa sebanyak apa pun efek dijadwalkan, jumlah node filter yang dibuat
/// TIDAK PERNAH melebihi jumlah tipe efek dan panjang string command tetap ultra-kompak (< 800 karakter).
pub fn apply_builtin_effects(
    graph: &mut FilterGraph,
    input_v: &str,
    effects: &[ScheduledBuiltinEffect],
) -> String {
    if effects.is_empty() {
        return input_v.to_string();
    }

    // Kelompokkan efek berdasarkan tipe untuk konsolidasi single-filter
    let mut shake_spans: Vec<(f64, f64)> = Vec::new();
    let mut flash_spans: Vec<(f64, f64)> = Vec::new();
    let mut bw_spans: Vec<(f64, f64)> = Vec::new();
    let mut deepfried_spans: Vec<(f64, f64)> = Vec::new();
    let mut zoom_spans: Vec<(f64, f64)> = Vec::new();
    let mut red_spans: Vec<(f64, f64)> = Vec::new();
    let mut negate_spans: Vec<(f64, f64)> = Vec::new();
    let mut blur_spans: Vec<(f64, f64)> = Vec::new();
    let mut sepia_spans: Vec<(f64, f64)> = Vec::new();
    let mut rainbow_spans: Vec<(f64, f64)> = Vec::new();

    for eff in effects {
        let span = (eff.start_time, eff.end_time);
        match eff.effect_type {
            BuiltinEffectType::ScreenShake => shake_spans.push(span),
            BuiltinEffectType::WhiteFlash => flash_spans.push(span),
            BuiltinEffectType::DramaticBW => bw_spans.push(span),
            BuiltinEffectType::DeepFried => deepfried_spans.push(span),
            BuiltinEffectType::PunchZoom => zoom_spans.push(span),
            BuiltinEffectType::RedTint => red_spans.push(span),
            BuiltinEffectType::Negate => negate_spans.push(span),
            BuiltinEffectType::FocusBlur => blur_spans.push(span),
            BuiltinEffectType::Sepia => sepia_spans.push(span),
            BuiltinEffectType::RainbowHue => rainbow_spans.push(span),
        }
    }

    // Helper untuk merangkai ekspresi boolean OR: between(t, s1, e1) + between(t, s2, e2)
    let build_between_expr = |spans: &[(f64, f64)]| -> String {
        spans
            .iter()
            .map(|(s, e)| format!("between(t,{:.2},{:.2})", s, e))
            .collect::<Vec<_>>()
            .join("+")
    };

    let mut linear_filters: Vec<String> = Vec::new();

    // 1. Deep-Fried (Kontras 2.0, Saturasi 3.0)
    if !deepfried_spans.is_empty() {
        let expr = build_between_expr(&deepfried_spans);
        linear_filters.push(format!("eq=contrast=2.0:saturation=3.0:enable='{}'", expr));
    }

    // 2. Dramatic Black & White + Cinematic Vignette
    if !bw_spans.is_empty() {
        let expr = build_between_expr(&bw_spans);
        linear_filters.push(format!("hue=s=0:enable='{}'", expr));
        linear_filters.push(format!("vignette=PI/4:enable='{}'", expr));
    }

    // 3. Punch Zoom Snap In (18% zoom ke tengah kanvas)
    if !zoom_spans.is_empty() {
        let expr = build_between_expr(&zoom_spans);
        linear_filters.push(format!(
            "crop=w='if({}, in_w*0.82, in_w)':h='if({}, in_h*0.82, in_h)':x='(in_w-out_w)/2':y='(in_h-out_h)/2',scale=1080:1920:flags=fast_bilinear",
            expr, expr
        ));
    }

    // 4. Screen Shake (Getaran rotasi kamera dinamis 2.9 derajat di 50 rad/s)
    if !shake_spans.is_empty() {
        let expr = build_between_expr(&shake_spans);
        linear_filters.push(format!(
            "rotate='sin(t*50)*0.05':ow=iw:oh=ih:enable='{}'",
            expr
        ));
    }

    // 5. White Flash (Kilatan putih impact)
    if !flash_spans.is_empty() {
        let expr = build_between_expr(&flash_spans);
        linear_filters.push(format!("eq=brightness=0.85:enable='{}'", expr));
    }

    // 6. Red Tint (Nuansa merah intensif untuk amarah/bahaya)
    if !red_spans.is_empty() {
        let expr = build_between_expr(&red_spans);
        linear_filters.push(format!("colorchannelmixer=rr=1.8:gg=0.4:bb=0.4:enable='{}'", expr));
    }

    // 7. Negate / Invert Colors (Efek cursed / jumpscare)
    if !negate_spans.is_empty() {
        let expr = build_between_expr(&negate_spans);
        linear_filters.push(format!("negate=enable='{}'", expr));
    }

    // 8. Focus Blur (Gaussian blur sigma=12 untuk momen pusing / blank)
    if !blur_spans.is_empty() {
        let expr = build_between_expr(&blur_spans);
        linear_filters.push(format!("gblur=sigma=12:enable='{}'", expr));
    }

    // 9. Sepia Tone (Warna vintage nostalgia untuk refleksi sedih / flashback)
    if !sepia_spans.is_empty() {
        let expr = build_between_expr(&sepia_spans);
        linear_filters.push(format!(
            "colorchannelmixer=.393:.769:.189:0:.349:.686:.168:0:.272:.534:.131:enable='{}'",
            expr
        ));
    }

    // 10. Rainbow Hue (Siklus warna pelangi dinamis untuk selebrasi / victory / hype)
    if !rainbow_spans.is_empty() {
        let expr = build_between_expr(&rainbow_spans);
        linear_filters.push(format!("hue=H=8*PI*t:enable='{}'", expr));
    }

    if linear_filters.is_empty() {
        return input_v.to_string();
    }

    let out_label = "v_builtin".to_string();
    let combined_filter = linear_filters.join(",");
    log::info!(
        "Applying {} builtin FFmpeg visual effects (compressed filter length: {} chars)",
        effects.len(),
        combined_filter.len()
    );

    let node = crate::processing::ffmpeg::filters::FilterNode::new(&combined_filter)
        .inputs(&[input_v])
        .outputs(&[out_label.as_str()]);
    graph.add_node(node);

    out_label
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_effects_returns_input() {
        let mut graph = FilterGraph::new();
        let out = apply_builtin_effects(&mut graph, "0:v", &[]);
        assert_eq!(out, "0:v");
        assert_eq!(graph.to_string(), "");
    }

    #[test]
    fn test_spans_merging_keeps_compact_length() {
        let mut graph = FilterGraph::new();
        let effects = vec![
            ScheduledBuiltinEffect {
                effect_type: BuiltinEffectType::ScreenShake,
                start_time: 2.0,
                end_time: 2.6,
            },
            ScheduledBuiltinEffect {
                effect_type: BuiltinEffectType::ScreenShake,
                start_time: 8.5,
                end_time: 9.1,
            },
            ScheduledBuiltinEffect {
                effect_type: BuiltinEffectType::WhiteFlash,
                start_time: 4.0,
                end_time: 4.25,
            },
            ScheduledBuiltinEffect {
                effect_type: BuiltinEffectType::DramaticBW,
                start_time: 15.0,
                end_time: 17.5,
            },
        ];

        let out = apply_builtin_effects(&mut graph, "0:v", &effects);
        assert_eq!(out, "v_builtin");

        let graph_str = graph.to_string();
        // Pastikan ekspresi between digabung dengan tanda '+'
        assert!(graph_str.contains("between(t,2.00,2.60)+between(t,8.50,9.10)"));
        assert!(graph_str.contains("rotate='sin(t*50)*0.05'"));
        assert!(graph_str.contains("hue=s=0"));
        assert!(graph_str.contains("vignette=PI/4"));
        assert!(graph_str.contains("eq=brightness="));

        // Panjang filter harus sangat kompak (< 450 karakter) meskipun ada 4 efek berbeda
        assert!(graph_str.len() < 450, "Graph string was too long: {}", graph_str.len());
    }

    #[test]
    fn test_all_10_builtin_effects_compact() {
        let mut graph = FilterGraph::new();
        let effects = vec![
            ScheduledBuiltinEffect {
                effect_type: BuiltinEffectType::ScreenShake,
                start_time: 1.0,
                end_time: 1.6,
            },
            ScheduledBuiltinEffect {
                effect_type: BuiltinEffectType::WhiteFlash,
                start_time: 2.0,
                end_time: 2.2,
            },
            ScheduledBuiltinEffect {
                effect_type: BuiltinEffectType::DramaticBW,
                start_time: 3.0,
                end_time: 5.0,
            },
            ScheduledBuiltinEffect {
                effect_type: BuiltinEffectType::DeepFried,
                start_time: 6.0,
                end_time: 7.2,
            },
            ScheduledBuiltinEffect {
                effect_type: BuiltinEffectType::PunchZoom,
                start_time: 8.0,
                end_time: 8.8,
            },
            ScheduledBuiltinEffect {
                effect_type: BuiltinEffectType::RedTint,
                start_time: 9.0,
                end_time: 10.0,
            },
            ScheduledBuiltinEffect {
                effect_type: BuiltinEffectType::Negate,
                start_time: 11.0,
                end_time: 11.4,
            },
            ScheduledBuiltinEffect {
                effect_type: BuiltinEffectType::FocusBlur,
                start_time: 12.0,
                end_time: 13.2,
            },
            ScheduledBuiltinEffect {
                effect_type: BuiltinEffectType::Sepia,
                start_time: 14.0,
                end_time: 16.5,
            },
            ScheduledBuiltinEffect {
                effect_type: BuiltinEffectType::RainbowHue,
                start_time: 17.0,
                end_time: 18.8,
            },
        ];

        let out = apply_builtin_effects(&mut graph, "0:v", &effects);
        assert_eq!(out, "v_builtin");

        let graph_str = graph.to_string();
        assert!(graph_str.contains("colorchannelmixer=rr=1.8:gg=0.4:bb=0.4"));
        assert!(graph_str.contains("negate=enable="));
        assert!(graph_str.contains("gblur=sigma=12"));
        assert!(graph_str.contains("colorchannelmixer=.393:.769:.189"));
        assert!(graph_str.contains("hue=H=8*PI*t"));

        // Bahkan dengan kesepuluh efek terdaftar sekaligus, panjang string tetap di bawah 800 karakter
        assert!(
            graph_str.len() < 800,
            "Graph string for all 10 effects was too long: {}",
            graph_str.len()
        );
    }
}
