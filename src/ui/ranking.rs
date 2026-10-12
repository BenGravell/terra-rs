use super::theme::{ACCENT, percent};
use crate::model::{Country, Score};
use bevy_egui::egui::{self, RichText};

pub(super) fn show(
    ui: &mut egui::Ui,
    countries: &[Country],
    scores: &[Score],
    selected: &mut Option<usize>,
) {
    let compact = ui.available_width() < 480.;
    ui.heading("Country ranking");
    ui.small("Rank among eligible countries · select a row to explore");
    egui::ScrollArea::both()
        .id_salt("ranking")
        .max_height(340.)
        .show(ui, |ui| {
            egui::Grid::new("ranking_table")
                .striped(true)
                .min_col_width(32.)
                .show(ui, |ui| {
                    let titles: &[&str] = if compact {
                        &["Rank", "Country", "Overall"]
                    } else {
                        &["Rank", "Country", "Overall", "Culture", "Life"]
                    };
                    for title in titles {
                        ui.strong(*title);
                    }
                    ui.end_row();
                    for s in scores.iter().filter(|s| s.matches) {
                        let country = &countries[s.country];
                        ui.label(s.rank.to_string());
                        if ui
                            .selectable_label(*selected == Some(s.country), &country.name)
                            .clicked()
                        {
                            *selected = Some(s.country);
                        }
                        ui.label(RichText::new(percent(s.overall)).color(ACCENT).strong());
                        if !compact {
                            ui.label(percent(s.culture));
                            ui.label(percent(s.quality));
                        }
                        ui.end_row();
                    }
                });
        });
}

#[cfg(test)]
#[path = "ranking_test.rs"]
mod tests;
