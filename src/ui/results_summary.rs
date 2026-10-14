use super::theme::ACCENT;
use crate::model::{Score, Settings};
use bevy_egui::egui::{self, RichText};

pub(super) fn show(ui: &mut egui::Ui, settings: &Settings, scores: &[Score]) -> bool {
    if let Err(error) = settings.validate() {
        ui.heading("Check your settings");
        ui.label(error);
        ui.label("Results will appear when all settings are valid.");
        return false;
    }
    let matches = scores.iter().filter(|s| s.matches).count();
    ui.horizontal_wrapped(|ui| {
        for (value, label) in [(matches, "Matches"), (scores.len(), "Eligible countries")] {
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.set_min_width(120.);
                ui.vertical(|ui| {
                    ui.label(RichText::new(value.to_string()).size(24.).color(ACCENT));
                    ui.label(label);
                });
            });
        }
    });
    if matches == 0 {
        ui.colored_label(
            super::theme::WARNING,
            "No countries match. Try lowering a minimum or allowing more continents.",
        );
    }
    true
}

#[cfg(test)]
#[path = "results_summary_test.rs"]
mod tests;
