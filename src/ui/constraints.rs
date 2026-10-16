use crate::model::{CONTINENTS, Settings};
use bevy_egui::egui;

pub(super) fn show(ui: &mut egui::Ui, settings: &mut Settings) {
    egui::CollapsingHeader::new("Constraints")
        .default_open(false)
        .show(ui, |ui| {
            content(ui, settings);
        });
}

pub(super) fn content(ui: &mut egui::Ui, settings: &mut Settings) {
    ui.label("Active limits are strict: a score must exceed its minimum and climate values must lie inside the range.");
    let minimums = &mut settings.minimums;
    for (label, value) in [
        ("Culture Fit minimum", &mut minimums.culture),
        ("Quality of Life minimum", &mut minimums.quality),
        ("Happy Planet minimum", &mut minimums.happy_planet),
        ("Social Progress minimum", &mut minimums.social_progress),
        ("Human Freedom minimum", &mut minimums.human_freedom),
    ] {
        percentage(ui, label, value);
    }
    percentage(ui, "English-speaking minimum", &mut settings.english_min);
    ui.label("Annual average temperature (°C)");
    for (name, value) in [
        ("Minimum °C", &mut settings.temperature.min),
        ("Maximum °C", &mut settings.temperature.max),
    ] {
        let label = ui.label(name);
        super::slider::show(ui, value, -10.0..=30.0, label.id, "", false);
    }
    ui.label("Daily sunshine (hours)");
    for (name, value) in [
        ("Minimum hours", &mut settings.sunshine.min),
        ("Maximum hours", &mut settings.sunshine.max),
    ] {
        let label = ui.label(name);
        super::slider::show(ui, value, 3.0..=10.0, label.id, "", false);
    }
    ui.label("Continents");
    for continent in CONTINENTS {
        let mut active = settings.continents.iter().any(|c| c == continent);
        if ui.checkbox(&mut active, continent).changed() {
            if active {
                settings.continents.push(continent.into());
            } else {
                settings.continents.retain(|c| c != continent);
            }
        }
    }
}

fn percentage(ui: &mut egui::Ui, label: &str, value: &mut f64) {
    let label = ui.label(label);
    let mut pct = *value * 100.;
    if super::slider::show(ui, &mut pct, 0.0..=100.0, label.id, "%", false).changed() {
        *value = pct / 100.;
    }
}
