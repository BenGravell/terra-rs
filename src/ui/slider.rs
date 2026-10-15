use bevy_egui::egui;
use std::ops::RangeInclusive;

pub(super) const VALUE_WIDTH: f32 = 64.;

pub(super) fn show(
    ui: &mut egui::Ui,
    value: &mut f64,
    range: RangeInclusive<f64>,
    label: egui::Id,
    suffix: &str,
) -> egui::Response {
    ui.scope(|ui| {
        ui.spacing_mut().slider_width =
            (ui.available_width() - VALUE_WIDTH - ui.spacing().item_spacing.x).max(0.);
        ui.spacing_mut().interact_size.x = VALUE_WIDTH;
        ui.spacing_mut().interact_size.y = ui.spacing().interact_size.y.max(
            ui.text_style_height(&ui.style().drag_value_text_style)
                + 2. * ui.spacing().button_padding.y,
        );
        ui.add(
            egui::Slider::new(value, range)
                .integer()
                .clamping(egui::SliderClamping::Edits)
                .suffix(suffix),
        )
        .labelled_by(label)
    })
    .inner
}

#[cfg(test)]
#[path = "slider_test.rs"]
mod tests;
