use super::theme::ACCENT;
use bevy_egui::egui::{self, RichText};

pub(super) fn show(ui: &mut egui::Ui, narrow: bool, settings_tab: &mut bool) {
    ui.add_space(6.);
    ui.horizontal_wrapped(|ui| {
        ui.label(
            RichText::new("TERRA")
                .font(egui::FontId::new(
                    25.,
                    egui::FontFamily::Name("caps".into()),
                ))
                .color(ACCENT),
        );
        if !narrow {
            ui.label("Find your place in the world");
        }
        if narrow {
            ui.selectable_value(settings_tab, true, "Settings");
            ui.selectable_value(settings_tab, false, "Results");
        }
    });
    ui.add_space(6.);
}
