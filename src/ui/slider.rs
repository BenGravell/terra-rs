use bevy_egui::egui;
use std::ops::RangeInclusive;

pub(super) const VALUE_WIDTH: f32 = 64.;

pub(super) fn show(
    ui: &mut egui::Ui,
    value: &mut f64,
    range: RangeInclusive<f64>,
    label: egui::Id,
    suffix: &str,
    signed: bool,
) -> egui::Response {
    ui.scope(|ui| {
        ui.spacing_mut().slider_width =
            (ui.available_width() - VALUE_WIDTH - ui.spacing().item_spacing.x).max(0.);
        ui.spacing_mut().interact_size.x = VALUE_WIDTH;
        ui.spacing_mut().interact_size.y = ui.spacing().interact_size.y.max(
            ui.text_style_height(&ui.style().drag_value_text_style)
                + 2. * ui.spacing().button_padding.y,
        );
        let mut slider = egui::Slider::new(value, range)
            .integer()
            .clamping(egui::SliderClamping::Edits)
            .suffix(suffix);
        if signed {
            slider = slider.custom_formatter(|value, _| {
                if value > 0. {
                    format!("+{value:.0}")
                } else {
                    format!("{value:.0}")
                }
            });
        }
        ui.add(slider).labelled_by(label)
    })
    .inner
}

#[cfg(test)]
#[path = "slider_test.rs"]
mod tests;
