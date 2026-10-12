use super::{constraints, culture, reference, weights};
use crate::model::{Country, Settings};
use bevy_egui::egui::{self, Color32};

pub(super) fn show(
    ui: &mut egui::Ui,
    countries: &[Country],
    settings: &mut Settings,
    reference: &mut Option<usize>,
    revision: &mut u64,
    message: Option<&str>,
) {
    // New IDs discard a focused numeric editor's old buffer after a bulk settings change.
    ui.push_id(*revision, |ui| {
        let mut reset = false;
        ui.horizontal(|ui| {
            ui.heading("Your preferences");
            reset = ui.button("Reset").clicked();
        });
        ui.label("Adjust what matters to you. Results update as you go.");
        if let Some(message) = message {
            ui.colored_label(Color32::LIGHT_YELLOW, message);
        }
        culture::show(ui, &mut settings.culture);
        weights::show(ui, settings);
        constraints::show(ui, settings);
        reference::show(ui, countries, settings, reference, revision);
        if reset {
            *revision += 1;
            *settings = Settings::default();
            *reference = None;
            ui.ctx().memory_mut(|memory| {
                if let Some(id) = memory.focused() {
                    memory.surrender_focus(id);
                }
            });
        }
        if let Err(message) = settings.validate() {
            ui.colored_label(Color32::LIGHT_YELLOW, message);
        }
    });
}

#[cfg(test)]
#[path = "settings_test.rs"]
mod tests;
