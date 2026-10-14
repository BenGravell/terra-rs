use super::{app::Tab, theme::ACCENT};
use bevy_egui::egui::{self, RichText};

pub(super) fn show(ui: &mut egui::Ui, narrow: bool, tab: &mut Tab) {
    ui.add_space(6.);
    ui.horizontal(|ui| {
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
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let (label, destination) = match tab {
                Tab::Results => ("About", Tab::About),
                Tab::About => ("Results", Tab::Results),
            };
            if ui.button(label).clicked() {
                *tab = destination;
            }
        });
    });
    ui.add_space(6.);
}
