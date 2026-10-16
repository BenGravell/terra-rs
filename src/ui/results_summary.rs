use crate::model::{Score, Settings};
use bevy_egui::egui;

pub(super) fn show(ui: &mut egui::Ui, settings: &Settings, scores: &[Score]) -> bool {
    if let Err(error) = settings.validate() {
        ui.heading("Check your settings");
        ui.label(error);
        ui.label("Results will appear when all settings are valid.");
        return false;
    }
    if !scores.iter().any(|score| score.matches) {
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
