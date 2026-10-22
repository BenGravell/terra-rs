use crate::model::Country;
use bevy_egui::egui;

#[derive(Default)]
pub(super) struct Selections {
    pub culture: Option<usize>,
    pub quality: Option<usize>,
}

pub(super) fn content(
    ui: &mut egui::Ui,
    countries: &[Country],
    reference: &mut Option<usize>,
    label: &str,
) -> bool {
    let search_id = ui.id().with("reference_country_search");
    let (mut search, was_open) = ui.ctx().data_mut(|data| {
        data.get_temp::<(String, bool)>(search_id)
            .unwrap_or_default()
    });
    let mut applied = false;
    let dropdown = egui::ComboBox::from_id_salt("reference_country")
        .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
        .width(ui.available_width())
        .truncate()
        .selected_text(label)
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
                    applied = true;
                    input.surrender_focus();
                    ui.close();
                }
            }
        });
    dropdown.response.on_hover_text(label);
    ui.ctx()
        .data_mut(|data| data.insert_temp(search_id, (search, dropdown.inner.is_some())));
    applied
}

#[cfg(test)]
#[path = "reference_test.rs"]
mod tests;
