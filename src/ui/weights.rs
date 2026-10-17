use crate::model::Settings;
use bevy_egui::egui;

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
    let overall = &mut settings.overall_importances;
    weight_slider(
        ui,
        "Culture Fit",
        "overall",
        &mut overall.culture,
        &mut [&mut overall.quality],
    );
    weight_slider(
        ui,
        "Quality of Life",
        "overall",
        &mut overall.quality,
        &mut [&mut overall.culture],
    );
    ui.separator();
    ui.label("Quality of Life");
    let quality = &mut settings.quality_importances;
    weight_slider(
        ui,
        "Happy Planet",
        "Quality of Life",
        &mut quality.happy_planet,
        &mut [&mut quality.social_progress, &mut quality.human_freedom],
    );
    weight_slider(
        ui,
        "Social Progress",
        "Quality of Life",
        &mut quality.social_progress,
        &mut [&mut quality.happy_planet, &mut quality.human_freedom],
    );
    weight_slider(
        ui,
        "Human Freedom",
        "Quality of Life",
        &mut quality.human_freedom,
        &mut [&mut quality.happy_planet, &mut quality.social_progress],
    );
}

fn weight_slider(
    ui: &mut egui::Ui,
    name: &str,
    target: &str,
    selected: &mut f64,
    peers: &mut [&mut f64],
) {
    let help = importance_help(name, target);
    if super::slider::row(ui, name, &help, selected, 0.0..=10.0, "", false).changed()
        && peers.iter().all(|value| **value == 0.)
    {
        *selected = selected.max(1.);
    }
}

fn importance_help(name: &str, target: &str) -> String {
    format!("How much {name} contributes to the {target} score.")
}

pub(super) fn reset(settings: &mut Settings) {
    let defaults = Settings::default();
    settings.overall_importances = defaults.overall_importances;
    settings.quality_importances = defaults.quality_importances;
}

#[cfg(test)]
#[path = "weights_test.rs"]
mod tests;
