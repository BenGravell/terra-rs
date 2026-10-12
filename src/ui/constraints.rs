use crate::model::{CONTINENTS, Settings};
use bevy_egui::egui;

pub(super) fn show(ui: &mut egui::Ui, settings: &mut Settings) {
    egui::CollapsingHeader::new("Constraints").default_open(false).show(ui, |ui| {
        ui.label("Active limits are strict: a score must exceed its minimum and climate values must lie inside the range.");
        let minimums = &mut settings.minimums;
        for (label, value) in [
            ("Culture Fit minimum", &mut minimums.culture),
            ("Quality of Life minimum", &mut minimums.quality),
            ("Happy Planet minimum", &mut minimums.happy_planet),
            ("Social Progress minimum", &mut minimums.social_progress),
            ("Human Freedom minimum", &mut minimums.human_freedom),
        ] {
            percentage(ui, label, value, 5.);
        }
        percentage(ui, "English-speaking minimum", &mut settings.english_min, 5.);
        ui.label("Annual average temperature (°C)");
        ui.add(egui::Slider::new(&mut settings.temperature.min, -10.0..=30.0).step_by(0.5).text("Minimum °C"));
        ui.add(egui::Slider::new(&mut settings.temperature.max, -10.0..=30.0).step_by(0.5).text("Maximum °C"));
        ui.label("Daily sunshine (hours)");
        ui.add(egui::Slider::new(&mut settings.sunshine.min, 3.0..=10.0).step_by(0.1).text("Minimum hours"));
        ui.add(egui::Slider::new(&mut settings.sunshine.max, 3.0..=10.0).step_by(0.1).text("Maximum hours"));
        ui.label("Continents");
        for continent in CONTINENTS {
            let mut active = settings.continents.iter().any(|c| c == continent);
            if ui.checkbox(&mut active, continent).changed() {
                if active { settings.continents.push(continent.into()); }
                else { settings.continents.retain(|c| c != continent); }
            }
        }
    });
}

fn percentage(ui: &mut egui::Ui, label: &str, value: &mut f64, step: f64) {
    let label = ui.label(label);
    let mut pct = *value * 100.;
    if ui
        .add(
            egui::Slider::new(&mut pct, 0.0..=100.0)
                .step_by(step)
                .suffix("%"),
        )
        .labelled_by(label.id)
        .changed()
    {
        *value = pct / 100.;
    }
}
