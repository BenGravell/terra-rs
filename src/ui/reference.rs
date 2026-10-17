use crate::model::{Country, Settings};
use bevy_egui::egui;

pub(super) fn show(
    ui: &mut egui::Ui,
    countries: &[Country],
    settings: &mut Settings,
    reference: &mut Option<usize>,
    revision: &mut u64,
) {
    super::icons::section("Presets", false).show(ui, |ui| {
        content(ui, countries, settings, reference, revision);
        if super::settings::reset_button(ui, "Presets") {
            *reference = None;
        }
    });
}

pub(super) fn content(
    ui: &mut egui::Ui,
    countries: &[Country],
    settings: &mut Settings,
    reference: &mut Option<usize>,
    revision: &mut u64,
) {
    let search_id = ui.id().with("reference_country_search");
    let (mut search, was_open) = ui.ctx().data_mut(|data| {
        data.get_temp::<(String, bool)>(search_id)
            .unwrap_or_default()
    });
    let dropdown = egui::ComboBox::from_id_salt("reference_country")
        .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
        .selected_text(
            reference
                .map(|i| countries[i].name.as_str())
                .unwrap_or("Choose a Reference Country"),
        )
        .show_ui(ui, |ui| {
            if !was_open {
                search.clear();
            }
            let input = ui.add(
                egui::TextEdit::singleline(&mut search)
                    .hint_text("Search Countries")
                    .desired_width(ui.available_width()),
            );
            if !was_open {
                input.request_focus();
            }
            let query = search.trim().to_lowercase();
            let mut countries: Vec<_> = countries
                .iter()
                .enumerate()
                .filter(|(_, c)| {
                    c.culture.is_some()
                        && c.quality.is_some()
                        && c.name.to_lowercase().contains(&query)
                })
                .collect();
            countries.sort_by_key(|(_, c)| &c.name);
            if countries.is_empty() {
                ui.label("No countries found.");
            }
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
                    input.surrender_focus();
                    ui.close();
                }
            }
        });
    ui.ctx()
        .data_mut(|data| data.insert_temp(search_id, (search, dropdown.inner.is_some())));
    ui.add_enabled_ui(reference.is_some(), |ui| {
        if ui.button("Use Culture Preferences").clicked() {
            *revision += 1;
            settings.use_culture(&countries[reference.unwrap()]);
        }
        if ui.button("Use Quality Minimums").clicked() {
            *revision += 1;
            settings.use_quality_thresholds(&countries[reference.unwrap()]);
        }
    });
}

#[cfg(test)]
#[path = "reference_test.rs"]
mod tests;
