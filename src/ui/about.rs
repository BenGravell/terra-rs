use super::theme;
use crate::model::Source;
use bevy_egui::egui;

pub(super) fn show(ui: &mut egui::Ui, sources: &[Source]) {
    let width = ui.available_width().min(800.);
    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        ui.allocate_ui_with_layout(
            egui::vec2(width, 0.),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                ui.set_width(width);
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
                ui.add_space(24.);
                ui.label(egui::RichText::new("ABOUT TERRA").small().color(theme::ACCENT));
                ui.heading("Which country feels like home?");
                let mut blurb = egui::text::LayoutJob::default();
                let format = egui::TextFormat {
                    font_id: egui::TextStyle::Body.resolve(ui.style()),
                    color: ui.visuals().text_color(),
                    ..Default::default()
                };
                for (text, italics) in [
                    ("Terra is a country recommendation app designed to help you ", false),
                    ("find your place in the world", true),
                    (" by using data to answer ", false),
                    ("\"which country is right for me?\"", true),
                ] {
                    blurb.append(text, 0., egui::TextFormat { italics, ..format.clone() });
                }
                ui.label(blurb);
                ui.add_space(16.);

                egui::Frame::group(ui.style()).inner_margin(16.).show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.heading("Start exploring");
                    ui.label("1. Set your culture preferences, or choose a reference country.");
                    ui.label("2. Adjust your weights and constraints to reflect what matters to you.");
                    ui.label("3. Compare the ranking and map, then select a country to explore its scores.");
                });
                ui.add_space(16.);
                ui.heading("Understand your results");
                ui.label("Open a topic to learn how Terra compares countries.");
                ui.add_space(4.);

                egui::CollapsingHeader::new("Culture Fit").show(ui, |ui| {
                    ui.label("Culture Fit is one minus the average distance from your culture preferences across the six national culture dimensions. A higher score means a closer match to your preferences.");
                });
                egui::CollapsingHeader::new("Quality of Life").show(ui, |ui| {
                    ui.label("Quality of Life combines Happy Planet, Social Progress, and Human Freedom using your weights. Adjust those weights to change the balance between the three indices.");
                });
                egui::CollapsingHeader::new("Overall score & ranking").show(ui, |ui| {
                    ui.label("The overall score combines Culture Fit and Quality of Life using your weights. Ranks number matching countries consecutively after constraints. Equal scores are ordered by country ID.");
                });
                egui::CollapsingHeader::new("Constraints & missing data").show(ui, |ui| {
                    ui.label("Constraints filter countries by your requirements for language, climate, and continent. Incomplete culture, quality-of-life, climate, or continent data excludes a country; an active English constraint also requires language data.");
                });
                egui::CollapsingHeader::new("Caveats & limitations").show(ui, |ui| {
                    ui.label("These historical datasets are not a current forecast. National averages cannot describe every person or place, and the underlying indices have limitations and biases. Use the results as a starting point for further research.");
                });
                ui.add_space(16.);
                ui.separator();
                ui.heading("Data sources");
                egui::ScrollArea::horizontal().id_salt("sources_table_scroll").show(ui, |ui| {
                egui::Grid::new("about_sources")
                    .num_columns(4)
                    .striped(true)
                    .min_col_width(80.)
                    .max_col_width(200.)
                    .spacing([20., 12.])
                    .show(ui, |ui| {
                        ui.strong("Name");
                        ui.strong("Year");
                        ui.strong("License");
                        ui.strong("Link");
                        ui.end_row();
                        for source in sources {
                            ui.label(&source.name);
                            ui.label(&source.year);
                            ui.label(&source.license);
                            ui.hyperlink_to("View source", &source.url)
                                .on_hover_text(&source.url);
                            ui.end_row();
                        }
                        ui.label("Map: Natural Earth 5.1.2");
                        ui.label("—");
                        ui.label("Public domain");
                        ui.hyperlink_to(
                            "View source",
                            "https://www.naturalearthdata.com/about/terms-of-use/",
                        );
                        ui.end_row();
                    });
                });
                ui.add_space(24.);
            },
        );
    });
}
