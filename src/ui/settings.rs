use super::{constraints, culture, reference, weights};
use crate::model::{Country, Settings};
use bevy_egui::egui;

pub(super) fn show(
    ui: &mut egui::Ui,
    countries: &[Country],
    settings: &mut Settings,
    reference: &mut Option<usize>,
    revision: &mut u64,
    message: Option<&str>,
) {
    if ui.ctx().content_rect().width() < 850. {
        let id = ui.id().with("preferences_tab");
        let mut tab = ui
            .ctx()
            .data_mut(|data| data.get_temp::<usize>(id).unwrap_or(0));
        ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
            super::layout::tab_selector(
                ui,
                &mut tab,
                &["Culture", "Weights", "Filters", "Presets"],
            );
            ui.separator();
            let reset = reset_button(ui, ["Culture", "Weights", "Filters", "Presets"][tab]);
            ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                egui::ScrollArea::vertical()
                    .id_salt(("preferences_content", tab))
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        show_content(
                            ui,
                            countries,
                            settings,
                            reference,
                            revision,
                            message,
                            Some(tab),
                        )
                    });
            });
            if reset {
                match tab {
                    0 => culture::reset(&mut settings.culture),
                    1 => weights::reset(settings),
                    2 => constraints::reset(settings),
                    _ => *reference = None,
                }
                *revision += 1;
            }
        });
        ui.ctx().data_mut(|data| data.insert_temp(id, tab));
    } else {
        show_content(ui, countries, settings, reference, revision, message, None);
    }
}

fn show_content(
    ui: &mut egui::Ui,
    countries: &[Country],
    settings: &mut Settings,
    reference: &mut Option<usize>,
    revision: &mut u64,
    message: Option<&str>,
    tab: Option<usize>,
) {
    // New IDs discard a focused numeric editor's old buffer after a bulk settings change.
    ui.push_id(*revision, |ui| {
        if let Some(message) = message {
            ui.colored_label(super::theme::WARNING, message);
        }
        match tab {
            Some(0) => culture::content(ui, &mut settings.culture),
            Some(1) => weights::content(ui, settings),
            Some(2) => constraints::content(ui, settings),
            Some(_) => reference::content(ui, countries, settings, reference, revision),
            None => {
                ui.spacing_mut().indent = 32.;
                let reset = culture::show(ui, &mut settings.culture)
                    | weights::show(ui, settings)
                    | constraints::show(ui, settings);
                if reset {
                    *revision += 1;
                }
                reference::show(ui, countries, settings, reference, revision);
            }
        }
        if let Err(message) = settings.validate() {
            ui.colored_label(super::theme::WARNING, message);
        }
    });
}

pub(super) fn reset_button(ui: &mut egui::Ui, section: &str) -> bool {
    let reset = ui
        .add_sized(
            [ui.available_width(), 33.],
            egui::Button::new(format!("Reset {section}")),
        )
        .clicked();
    if reset {
        ui.ctx().memory_mut(|memory| {
            if let Some(id) = memory.focused() {
                memory.surrender_focus(id);
            }
        });
    }
    reset
}

#[cfg(test)]
#[path = "settings_test.rs"]
mod tests;
