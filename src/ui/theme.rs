use bevy_egui::egui::{self, Color32, Vec2 as EVec2};
use std::sync::Arc;

// Muted blue and warm earth tones; neutral surfaces keep attention on the data.
pub(super) const ACCENT: Color32 = Color32::from_rgb(48, 86, 110);
pub(super) const CULTURE: Color32 = Color32::from_rgb(229, 181, 27);
pub(super) const WARNING: Color32 = Color32::from_rgb(137, 96, 35);
pub(super) fn quality_colors() -> [Color32; 3] {
    // Fixed samples span indigo, blue-teal, and green on the shared map scale.
    [0.90, 0.55, 0.25].map(score_color)
}

pub(super) const BACKGROUND: Color32 = Color32::from_rgb(248, 247, 244);
pub(super) const SURFACE: Color32 = Color32::from_rgb(253, 252, 249);
pub(super) const TEXT: Color32 = Color32::from_rgb(48, 54, 58);
pub(super) const TRACK: Color32 = Color32::from_rgb(226, 227, 224);
pub(super) const BORDER: Color32 = Color32::from_rgb(202, 207, 207);
pub(super) const MAP_BACKGROUND: Color32 = Color32::from_rgb(239, 242, 241);
pub(super) const INFEASIBLE: Color32 = Color32::from_rgb(164, 157, 146);
pub(super) const NO_ELIGIBLE_DATA: Color32 = Color32::from_rgb(218, 217, 212);

pub(super) const QUALITY_NAMES: [&str; 3] = ["Happy Planet", "Social Progress", "Human Freedom"];

pub(super) fn apply(ctx: &egui::Context) {
    egui_extras::install_image_loaders(ctx);
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
    // Filtering legitimately replaces country identities at the same row position.
    style.debug.warn_if_rect_changes_id = false;
    // Format slider readouts without rounding their stored values.
    style.number_formatter = egui::style::NumberFormatter::new(|value, _| format!("{value:.0}"));
    style.visuals = egui::Visuals::light();
    style.visuals.panel_fill = BACKGROUND;
    style.visuals.window_fill = SURFACE;
    style.visuals.extreme_bg_color = MAP_BACKGROUND;
    style.visuals.faint_bg_color = Color32::from_rgb(239, 238, 234);
    style.visuals.override_text_color = Some(TEXT);
    style.visuals.weak_text_color = Some(Color32::from_rgb(101, 108, 110));
    style.visuals.selection.bg_fill = Color32::from_rgb(216, 229, 234);
    style.visuals.selection.stroke = egui::Stroke::new(1., ACCENT);
    style.visuals.hyperlink_color = ACCENT;
    style.visuals.warn_fg_color = WARNING;
    style.visuals.error_fg_color = Color32::from_rgb(163, 77, 53);
    style.visuals.window_stroke = egui::Stroke::new(1., BORDER);
    for (widget, fill, stroke) in [
        (
            &mut style.visuals.widgets.noninteractive,
            BACKGROUND,
            BORDER,
        ),
        (
            &mut style.visuals.widgets.inactive,
            TRACK,
            Color32::from_rgb(125, 137, 142),
        ),
        (
            &mut style.visuals.widgets.hovered,
            Color32::from_rgb(224, 234, 237),
            ACCENT,
        ),
        (
            &mut style.visuals.widgets.active,
            Color32::from_rgb(202, 221, 229),
            ACCENT,
        ),
        (
            &mut style.visuals.widgets.open,
            Color32::from_rgb(224, 234, 237),
            ACCENT,
        ),
    ] {
        widget.bg_fill = fill;
        widget.weak_bg_fill = fill;
        widget.bg_stroke.color = stroke;
        widget.fg_stroke.color = TEXT;
    }
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
    const DEEP: [[u8; 3]; 256] = include!("../../assets/colormaps/deep.rs");
    let position = (0.10 + 0.80 * score.clamp(0., 1.)) * 255.;
    let index = position.floor() as usize;
    let fraction = position.fract();
    let rgb: [u8; 3] = std::array::from_fn(|channel| {
        let start = f64::from(DEEP[index][channel]);
        let end = f64::from(DEEP[index + 1][channel]);
        (start + (end - start) * fraction).round() as u8
    });
    Color32::from_rgb(rgb[0], rgb[1], rgb[2])
}

pub(super) fn title(ui: &mut egui::Ui) -> egui::Response {
    let mut title = ui.painter().layout_no_wrap(
        "TERRA".into(),
        egui::FontId::new(25., egui::FontFamily::Name("caps".into())),
        egui::Color32::WHITE,
    );
    let bounds = title.mesh_bounds;
    for row in &mut Arc::make_mut(&mut title).rows {
        let offset = row.pos.x;
        for vertex in &mut Arc::make_mut(&mut row.row).visuals.mesh.vertices {
            let t = ((offset + vertex.pos.x - bounds.left()) / bounds.width()).clamp(0., 1.);
            vertex.color = score_color(0.45 + 0.35 * f64::from(t));
        }
    }
    ui.label(title)
}

#[cfg(test)]
#[path = "theme_test.rs"]
mod tests;
