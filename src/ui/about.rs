use crate::model::Source;
use bevy_egui::egui;

pub(super) fn show(ui: &mut egui::Ui, sources: &[Source]) {
    egui::CollapsingHeader::new("About the scores & sources").show(ui, |ui| {
        ui.label("Culture Fit is one minus the average distance from your culture preferences. Quality of Life combines Happy Planet, Social Progress, and Human Freedom. Overall score combines Culture Fit and Quality of Life using your weights.");
        ui.label("Ranks are among eligible countries before constraints. Ties share a rank. Incomplete culture, quality-of-life, climate, or continent data excludes a country; an active English constraint also requires language data. These historical datasets are not a current forecast.");
        for source in sources {
            ui.hyperlink_to(format!("{} · {}", source.name, source.year), &source.url);
        }
        ui.hyperlink_to("Map: Natural Earth 5.1.2, public domain", "https://www.naturalearthdata.com/about/terms-of-use/");
    });
}
