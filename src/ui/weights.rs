use crate::model::Settings;
use bevy_egui::egui;

const BALANCE_LABEL: &str = "Culture Fit vs Quality of Life";

pub(super) fn show(ui: &mut egui::Ui, settings: &mut Settings) -> bool {
    super::icons::section("Weights", true)
        .show(ui, |ui| {
            content(ui, settings);
            let clicked = super::settings::reset_button(ui, "Weights");
            if clicked {
                reset(settings);
            }
            clicked
        })
        .body_returned
        .unwrap_or(false)
}

pub(super) fn content(ui: &mut egui::Ui, settings: &mut Settings) {
    ui.label("Overall Score");
    let share = settings.quality_weight();
    let mut quality = if share.is_finite() {
        (share * 10.).clamp(0., 10.)
    } else {
        5.
    };
    let slider = ui
        .scope(|ui| {
            ui.spacing_mut().slider_width = ui.available_width();
            ui.add(
                egui::Slider::new(&mut quality, 0. ..=10.)
                    .integer()
                    .clamping(egui::SliderClamping::Edits)
                    .show_value(false)
                    .handle_shape(egui::style::HandleShape::Circle),
            )
        })
        .inner
        .on_hover_text("Left: only Culture Fit. Right: only Quality of Life.");
    slider.widget_info(|| egui::WidgetInfo::slider(ui.is_enabled(), quality, BALANCE_LABEL));
    if slider.changed() {
        settings.overall_importances.culture = 10. - quality;
        settings.overall_importances.quality = quality;
    }
    let (labels, _) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 40.), egui::Sense::hover());
    let size = egui::vec2((labels.width() / 2.).min(140.), 40.);
    for (name, rect, share, value) in [
        (
            "Culture Fit",
            egui::Rect::from_min_size(labels.min, size),
            1. - quality / 10.,
            0.,
        ),
        (
            "Quality of Life",
            egui::Rect::from_min_size(labels.right_top() - egui::vec2(size.x, 0.), size),
            quality / 10.,
            10.,
        ),
    ] {
        if super::icons::preference(ui, rect, name, share).clicked() {
            settings.overall_importances.culture = 10. - value;
            settings.overall_importances.quality = value;
        }
    }
    ui.separator();
    ui.label("Quality of Life");
    super::quality_triangle::show(ui, &mut settings.quality_importances);
}

pub(super) fn reset(settings: &mut Settings) {
    let defaults = Settings::default();
    settings.overall_importances = defaults.overall_importances;
    settings.quality_importances = defaults.quality_importances;
}

#[cfg(test)]
#[path = "weights_test.rs"]
mod tests;
