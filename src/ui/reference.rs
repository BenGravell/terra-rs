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
                let flags = super::flags::texture(ui);
                let flag_ids: Vec<_> = super::flags::FLAG_IDS.lines().collect();
                for (i, c) in countries {
                    let index = flag_ids
                        .iter()
                        .position(|id| *id == c.id)
                        .expect("reference country has a bundled flag");
                    let uv = super::flags::cell_uv(index);
                    let flag = egui::Image::new((flags.id(), egui::vec2(24., 18.))).uv(uv);
                    if ui
                        .add(egui::Button::selectable(
                            *reference == Some(i),
                            (flag, c.name.as_str()),
                        ))
                        .clicked()
                    {
                        *reference = Some(i);
                    }
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

#[cfg(test)]
mod tests {
    #[test]
    fn every_reference_country_has_a_flag() {
        let data =
            crate::model::Dataset::parse(include_str!("../../assets/countries.json")).unwrap();
        for country in data
            .countries
            .iter()
            .filter(|c| c.culture.is_some() && c.quality.is_some())
        {
            assert!(
                super::super::flags::FLAG_IDS
                    .lines()
                    .any(|id| id == country.id),
                "Missing flag for {}",
                country.name
            );
        }
    }
}
