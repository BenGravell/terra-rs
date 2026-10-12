use super::theme::ACCENT;
use crate::model::{Score, Settings};
use bevy_egui::egui::{self, Color32, RichText};

pub(super) fn show(ui: &mut egui::Ui, settings: &Settings, scores: &[Score]) -> bool {
    if let Err(error) = settings.validate() {
        ui.heading("Check your settings");
        ui.label(error);
        ui.label("Results will appear when all settings are valid.");
        return false;
    }
    let matches = scores.iter().filter(|s| s.matches).count();
    ui.horizontal_wrapped(|ui| {
        ui.heading("Your world, ranked");
        ui.label(
            RichText::new(format!(
                "{matches} matches · {} eligible countries",
                scores.len()
            ))
            .color(ACCENT),
        );
    });
    ui.label("A comparison based on your preferences and the pinned source datasets.");
    if matches == 0 {
        ui.colored_label(
            Color32::LIGHT_YELLOW,
            "No countries match. Try lowering a minimum or allowing more continents.",
        );
    }
    true
}

#[cfg(test)]
#[path = "results_summary_test.rs"]
mod tests;
