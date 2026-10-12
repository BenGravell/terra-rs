use crate::model::{Country, Settings};
use bevy_egui::egui;

pub(super) fn show(
    ui: &mut egui::Ui,
    countries: &[Country],
    settings: &mut Settings,
    reference: &mut Option<usize>,
    revision: &mut u64,
) {
    egui::CollapsingHeader::new("Start from a country").show(ui, |ui| {
        egui::ComboBox::from_id_salt("reference_country")
            .selected_text(
                reference
                    .map(|i| countries[i].name.as_str())
                    .unwrap_or("Choose a reference country"),
            )
            .show_ui(ui, |ui| {
                let mut countries: Vec<_> = countries
                    .iter()
                    .enumerate()
                    .filter(|(_, c)| c.culture.is_some() && c.quality.is_some())
                    .collect();
                countries.sort_by_key(|(_, c)| &c.name);
                for (i, c) in countries {
                    ui.selectable_value(reference, Some(i), &c.name);
                }
            });
        ui.add_enabled_ui(reference.is_some(), |ui| {
            if ui.button("Use culture preferences").clicked() {
                *revision += 1;
                settings.use_culture(&countries[reference.unwrap()]);
            }
            if ui.button("Use quality minimums").clicked() {
                *revision += 1;
                settings.use_quality_thresholds(&countries[reference.unwrap()]);
            }
        });
    });
}
