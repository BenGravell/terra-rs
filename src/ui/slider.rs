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
        let value_width = if ui.ctx().content_rect().width() < 850. {
            40.
        } else {
            VALUE_WIDTH
        };
        if ui.ctx().content_rect().width() < 850. {
            ui.spacing_mut().button_padding.x = 4.;
        }
        ui.spacing_mut().slider_width =
            (ui.available_width() - value_width - ui.spacing().item_spacing.x).max(0.);
        ui.spacing_mut().interact_size.x = value_width;
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

/// Keep mobile labels, help buttons, sliders and readouts in aligned columns.
pub(super) fn row(
    ui: &mut egui::Ui,
    name: &str,
    help: &str,
    value: &mut f64,
    range: RangeInclusive<f64>,
    suffix: &str,
    signed: bool,
) -> egui::Response {
    let show = |ui: &mut egui::Ui| {
        let label = super::help::label(ui, name, help);
        show(ui, value, range, label.id, suffix, signed)
    };
    if ui.ctx().content_rect().width() < 850. {
        ui.scope(|ui| {
            ui.style_mut()
                .text_styles
                .insert(egui::TextStyle::Body, egui::FontId::proportional(11.));
            ui.spacing_mut().item_spacing = egui::vec2(4., 2.);
            ui.spacing_mut().interact_size.y = 32.;
            ui.horizontal(show).inner
        })
        .inner
    } else {
        show(ui)
    }
}
