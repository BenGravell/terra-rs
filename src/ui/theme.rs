use bevy_egui::egui::{self, Color32, Vec2 as EVec2};

pub(super) const ACCENT: Color32 = Color32::from_rgb(132, 216, 178);
pub(super) const CULTURE: Color32 = Color32::from_rgb(137, 172, 241);
pub(super) const QUALITY: Color32 = ACCENT;
pub(super) const PARTS: [Color32; 3] = [
    Color32::from_rgb(233, 184, 111),
    Color32::from_rgb(130, 208, 204),
    Color32::from_rgb(190, 164, 233),
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
    let mut style = (*ctx.style_of(egui::Theme::Dark)).clone();
    // Format slider readouts without rounding their stored values.
    style.number_formatter = egui::style::NumberFormatter::new(|value, _| format!("{value:.0}"));
    style.visuals = egui::Visuals::dark();
    style.visuals.panel_fill = Color32::from_rgb(16, 29, 43);
    style.visuals.window_fill = Color32::from_rgb(23, 39, 54);
    style.visuals.extreme_bg_color = Color32::from_rgb(12, 24, 36);
    style.visuals.override_text_color = Some(Color32::from_rgb(227, 234, 238));
    style.visuals.selection.bg_fill = Color32::from_rgb(43, 83, 79);
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
    ctx.set_theme(egui::Theme::Dark);
    ctx.set_style_of(egui::Theme::Dark, style);
}

pub(super) fn percent(value: f64) -> String {
    format!("{:.1}%", value * 100.)
}
pub(super) fn score_color(score: f64) -> Color32 {
    let t = score.clamp(0., 1.) as f32;
    Color32::from_rgb(
        (65. + t * 99.) as u8,
        (87. + t * 142.) as u8,
        (120. + t * 67.) as u8,
    )
}
