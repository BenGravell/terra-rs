use bevy_egui::egui::{self, Color32, Vec2 as EVec2};

pub(super) const ACCENT: Color32 = Color32::from_rgb(30, 99, 233);
pub(super) const CULTURE: Color32 = Color32::from_rgb(180, 35, 73);
pub(super) const WARNING: Color32 = Color32::from_rgb(143, 82, 16);
pub(super) const PARTS: [Color32; 3] = [
    Color32::from_rgb(30, 99, 233),
    Color32::from_rgb(25, 145, 190),
    Color32::from_rgb(10, 119, 126),
];
pub(super) const QUALITY_NAMES: [&str; 3] = ["Happy Planet", "Social Progress", "Human Freedom"];

pub(super) fn apply(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "atkinson".into(),
        egui::FontData::from_static(include_bytes!(
            "../../assets/fonts/AtkinsonHyperlegibleNext/AtkinsonHyperlegibleNext.ttf"
        ))
        .into(),
    );
    fonts.font_data.insert(
        "atkinson_mono".into(),
        egui::FontData::from_static(include_bytes!(
            "../../assets/fonts/AtkinsonHyperlegibleMono/AtkinsonHyperlegibleMono.ttf"
        ))
        .into(),
    );
    let mut caps = egui::FontData::from_static(include_bytes!(
        "../../assets/fonts/SpaceGrotesk/SpaceGrotesk.ttf"
    ));
    caps.tweak.coords = egui::epaint::text::VariationCoords::new([(b"wght", 700.0)]);
    fonts
        .font_data
        .insert("space_grotesk_bold".into(), caps.into());
    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "atkinson".into());
    fonts
        .families
        .get_mut(&egui::FontFamily::Monospace)
        .unwrap()
        .insert(0, "atkinson_mono".into());
    fonts.families.insert(
        egui::FontFamily::Name("caps".into()),
        vec!["space_grotesk_bold".into(), "atkinson".into()],
    );
    ctx.set_fonts(fonts);
    let mut style = (*ctx.style_of(egui::Theme::Light)).clone();
    // Format slider readouts without rounding their stored values.
    style.number_formatter = egui::style::NumberFormatter::new(|value, _| format!("{value:.0}"));
    style.visuals = egui::Visuals::light();
    style.visuals.panel_fill = Color32::from_rgb(247, 249, 253);
    style.visuals.window_fill = Color32::from_rgb(255, 255, 255);
    style.visuals.extreme_bg_color = Color32::from_rgb(237, 242, 250);
    style.visuals.override_text_color = Some(Color32::from_rgb(30, 41, 63));
    style.visuals.selection.bg_fill = Color32::from_rgb(218, 231, 255);
    style.visuals.selection.stroke = egui::Stroke::new(1., ACCENT);
    style.visuals.hyperlink_color = ACCENT;
    style.spacing.item_spacing = EVec2::new(10., 8.);
    style.spacing.button_padding = EVec2::new(10., 6.);
    style
        .text_styles
        .insert(egui::TextStyle::Body, egui::FontId::proportional(16.));
    style
        .text_styles
        .insert(egui::TextStyle::Button, egui::FontId::proportional(15.));
    style
        .text_styles
        .insert(egui::TextStyle::Monospace, egui::FontId::monospace(14.));
    style.text_styles.insert(
        egui::TextStyle::Heading,
        egui::FontId::new(20., egui::FontFamily::Name("caps".into())),
    );
    ctx.set_theme(egui::Theme::Light);
    ctx.set_style_of(egui::Theme::Light, style);
}

pub(super) fn percent(value: f64) -> String {
    format!("{:.0}%", value * 100.)
}
pub(super) fn score_color(score: f64) -> Color32 {
    // Orchid keeps the red-to-blue transition colorful instead of muddy.
    let stops = [[180., 35., 73.], [171., 67., 190.], [30., 99., 233.]];
    let t = score.clamp(0., 1.) as f32 * 2.;
    let segment = (t as usize).min(1);
    let fraction = t - segment as f32;
    let rgb = std::array::from_fn::<_, 3, _>(|i| {
        (stops[segment][i] + (stops[segment + 1][i] - stops[segment][i]) * fraction) as u8
    });
    Color32::from_rgb(rgb[0], rgb[1], rgb[2])
}

#[cfg(test)]
#[path = "theme_test.rs"]
mod tests;
