use super::*;

fn luminance(color: Color32) -> f64 {
    let linear = [color.r(), color.g(), color.b()].map(|channel| {
        let value = f64::from(channel) / 255.;
        if value <= 0.04045 {
            value / 12.92
        } else {
            ((value + 0.055) / 1.055).powf(2.4)
        }
    });
    linear[0] * 0.2126 + linear[1] * 0.7152 + linear[2] * 0.0722
}

#[test]
fn score_gradient_is_clamped_and_darkens_with_increasing_score() {
    assert_eq!(score_color(-1.), score_color(0.));
    assert_eq!(score_color(2.), score_color(1.));
    for step in 1..=100 {
        assert!(
            luminance(score_color(f64::from(step) / 100.))
                < luminance(score_color(f64::from(step - 1) / 100.))
        );
    }
}

#[test]
fn text_colors_have_readable_contrast_on_surfaces() {
    let ctx = egui::Context::default();
    apply(&ctx);
    let style = ctx.style_of(egui::Theme::Light);
    for foreground in [
        TEXT,
        ACCENT,
        style.visuals.error_fg_color,
        WARNING,
        style.visuals.weak_text_color.unwrap(),
    ] {
        for background in [BACKGROUND, SURFACE] {
            let contrast = (luminance(background) + 0.05) / (luminance(foreground) + 0.05);
            assert!(
                contrast >= 4.5,
                "{foreground:?} on {background:?}: {contrast}"
            );
        }
    }
}

#[test]
fn score_gradient_has_no_large_perceptual_jumps() {
    use bevy::color::{Oklaba, Srgba};

    let lab = |score| {
        let color = score_color(score);
        Oklaba::from(Srgba::rgb_u8(color.r(), color.g(), color.b()))
    };
    let mut distances = Vec::new();
    for step in 1..=100 {
        let previous = lab(f64::from(step - 1) / 100.);
        let current = lab(f64::from(step) / 100.);
        distances.push(
            ((current.lightness - previous.lightness).powi(2)
                + (current.a - previous.a).powi(2)
                + (current.b - previous.b).powi(2))
            .sqrt(),
        );
    }
    let min = distances.iter().copied().fold(f32::INFINITY, f32::min);
    let max = distances.iter().copied().fold(0., f32::max);
    // deep is perceptually designed in CAM02-UCS; OKLab is a continuity check,
    // not a requirement to reproduce the old constant-OKLCH step sizes.
    assert!(min > 0. && max < 0.01, "perceptual steps: {min}..{max}");
}

#[test]
fn stacked_bars_separate_warm_culture_from_cool_quality() {
    assert!(CULTURE.r() > CULTURE.b());
    let [indigo, teal, green] = quality_colors();
    assert!(indigo.b() > indigo.g());
    assert!(teal.b() > teal.r() && teal.g() > teal.r());
    assert!(green.g() > green.b() && green.g() > green.r());
    assert!(luminance(teal) - luminance(indigo) > 0.08);
    assert!(luminance(green) - luminance(teal) > 0.08);
}

#[test]
fn score_gradient_uses_the_middle_of_cmocean_deep() {
    assert_eq!(score_color(0.0), Color32::from_rgb(201, 235, 178));
    assert_eq!(score_color(0.5), Color32::from_rgb(72, 143, 158));
    assert_eq!(score_color(1.0), Color32::from_rgb(56, 45, 82));
}
