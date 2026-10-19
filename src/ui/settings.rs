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
    show_content(ui, countries, settings, reference, revision, message, None);
}

pub(super) fn mobile_content(
    ui: &mut egui::Ui,
    countries: &[Country],
    settings: &mut Settings,
    reference: &mut Option<usize>,
    revision: &mut u64,
    message: Option<&str>,
    tab: usize,
) {
    ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
        let reset = reset_button(ui, ["Culture", "Weights", "Filters", "Presets"][tab]);
        let height_id = ui.id().with(("preferences_height", tab));
        let content_height = ui.ctx().data_mut(|data| {
            data.get_temp::<f32>(height_id)
                .unwrap_or(ui.available_height())
        });
        let size = egui::vec2(
            ui.available_width(),
            content_height.min(ui.available_height()),
        );
        ui.allocate_ui_with_layout(size, egui::Layout::top_down(egui::Align::Min), |ui| {
            let scroll = egui::ScrollArea::vertical()
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
                    );
                });
            if (scroll.content_size.y - content_height).abs() > 0.5 {
                ui.ctx()
                    .data_mut(|data| data.insert_temp(height_id, scroll.content_size.y));
                ui.ctx().request_discard("settings content height changed");
            }
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
