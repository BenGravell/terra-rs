use crate::model::{Settings, adjust_weight};
use bevy_egui::egui;

pub(super) fn show(ui: &mut egui::Ui, settings: &mut Settings) {
    egui::CollapsingHeader::new("Score weights")
        .default_open(true)
        .show(ui, |ui| {
            ui.label("Overall score");
            let overall = &mut settings.overall_weights;
            weight_slider(
                ui,
                "Culture Fit weight",
                &mut overall.culture,
                &mut [&mut overall.quality],
            );
            weight_slider(
                ui,
                "Quality of Life weight",
                &mut overall.quality,
                &mut [&mut overall.culture],
            );
            ui.separator();
            ui.label("Within Quality of Life");
            let quality = &mut settings.quality_weights;
            weight_slider(
                ui,
                "Happy Planet weight",
                &mut quality.happy_planet,
                &mut [&mut quality.social_progress, &mut quality.human_freedom],
            );
            weight_slider(
                ui,
                "Social Progress weight",
                &mut quality.social_progress,
                &mut [&mut quality.happy_planet, &mut quality.human_freedom],
            );
            weight_slider(
                ui,
                "Human Freedom weight",
                &mut quality.human_freedom,
                &mut [&mut quality.happy_planet, &mut quality.social_progress],
            );
        });
}

fn weight_slider(ui: &mut egui::Ui, label: &str, selected: &mut f64, peers: &mut [&mut f64]) {
    let label = ui.label(label);
    let mut pct = *selected * 100.;
    if ui
        .add(
            egui::Slider::new(&mut pct, 0.0..=100.0)
                .step_by(5.)
                // Redistribution may leave peers between steps; don't round them on idle frames.
                .clamping(egui::SliderClamping::Edits)
                .suffix("%"),
        )
        .labelled_by(label.id)
        .changed()
    {
        adjust_weight(selected, peers, pct / 100.).expect("UI weight values are bounded");
    }
}

#[cfg(test)]
#[path = "weights_test.rs"]
mod tests;
