use crate::model::Settings;
use bevy_egui::egui;

pub(super) fn show(ui: &mut egui::Ui, settings: &mut Settings) {
    egui::CollapsingHeader::new("Score importances")
        .default_open(true)
        .show(ui, |ui| {
            ui.label("Overall score");
            let overall = &mut settings.overall_importances;
            weight_slider(
                ui,
                "Culture Fit importance",
                &mut overall.culture,
                &mut [&mut overall.quality],
            );
            weight_slider(
                ui,
                "Quality of Life importance",
                &mut overall.quality,
                &mut [&mut overall.culture],
            );
            ui.separator();
            ui.label("Quality of Life");
            let quality = &mut settings.quality_importances;
            weight_slider(
                ui,
                "Happy Planet importance",
                &mut quality.happy_planet,
                &mut [&mut quality.social_progress, &mut quality.human_freedom],
            );
            weight_slider(
                ui,
                "Social Progress importance",
                &mut quality.social_progress,
                &mut [&mut quality.happy_planet, &mut quality.human_freedom],
            );
            weight_slider(
                ui,
                "Human Freedom importance",
                &mut quality.human_freedom,
                &mut [&mut quality.happy_planet, &mut quality.social_progress],
            );
        });
}

fn weight_slider(ui: &mut egui::Ui, label: &str, selected: &mut f64, peers: &mut [&mut f64]) {
    let label = ui.label(label);
    if super::slider::show(ui, selected, 0.0..=10.0, label.id, "").changed()
        && peers.iter().all(|value| **value == 0.)
    {
        *selected = selected.max(1.);
    }
}

#[cfg(test)]
#[path = "weights_test.rs"]
mod tests;
