use super::theme::percent;
use bevy_egui::egui::{self, Color32, Stroke, Vec2 as EVec2};

pub(super) fn stacked_bar(
    ui: &mut egui::Ui,
    size: EVec2,
    values: &[f64],
    colors: &[Color32],
    selected: bool,
    names: &[&str],
) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::hover());
    ui.painter()
        .rect_filled(rect, 2., Color32::from_rgb(228, 234, 244));
    let mut x = rect.left();
    for (value, color) in values.iter().zip(colors) {
        let w = rect.width() * *value as f32;
        ui.painter().rect_filled(
            egui::Rect::from_min_size(egui::pos2(x, rect.top()), EVec2::new(w, rect.height())),
            1.,
            *color,
        );
        x += w;
    }
    if selected {
        ui.painter().rect_stroke(
            rect,
            2.,
            Stroke::new(1., ui.visuals().text_color()),
            egui::StrokeKind::Outside,
        );
    }
    response.on_hover_text(
        names
            .iter()
            .zip(values)
            .map(|(name, value)| format!("{name}: {}", percent(*value)))
            .collect::<Vec<_>>()
            .join("\n"),
    )
}
