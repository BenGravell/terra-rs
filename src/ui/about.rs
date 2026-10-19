use crate::model::Source;
use bevy_egui::egui;

pub(super) fn show(ui: &mut egui::Ui, sources: &[Source]) {
    let width = ui.available_width();
    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
    ui.add_space(24.);
    ui.heading("About Terra");
    ui.label("Terra is a country recommendation app designed to help you find your place in the world by using data to answer \"which country is right for me?\"");
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

    let spacing = 12.;
    let content_width = (width - 2. * spacing).max(0.);
    // Keep every column within the page so touch scrolling belongs to the page.
    let columns = [
        content_width * 0.45,
        content_width * 0.18,
        content_width * 0.37,
    ];
    egui::Grid::new("about_sources")
        .num_columns(3)
        .striped(true)
        .min_col_width(0.)
        .spacing([spacing, 12.])
        .show(ui, |ui| {
            for (heading, width) in ["Name", "Year", "License"].into_iter().zip(columns) {
                cell(
                    ui,
                    width,
                    egui::Label::new(egui::RichText::new(heading).strong()).wrap(),
                );
            }
            ui.end_row();
            for source in sources {
                cell(
                    ui,
                    columns[0],
                    egui::Hyperlink::from_label_and_url(&source.name, &source.url),
                )
                .on_hover_text(&source.url);
                cell(ui, columns[1], egui::Label::new(&source.year).wrap());
                cell(ui, columns[2], egui::Label::new(&source.license).wrap());
                ui.end_row();
            }
            cell(
                ui,
                columns[0],
                egui::Hyperlink::from_label_and_url(
                    "Map: Natural Earth 5.1.2",
                    "https://www.naturalearthdata.com/about/terms-of-use/",
                ),
            );
            cell(ui, columns[1], egui::Label::new("—").wrap());
            cell(ui, columns[2], egui::Label::new("Public domain").wrap());
            ui.end_row();
        });
    ui.add_space(24.);
}

fn cell(ui: &mut egui::Ui, width: f32, widget: impl egui::Widget) -> egui::Response {
    // Start at the row top; allocating zero height would center the cell before it grows.
    ui.scope_builder(
        egui::UiBuilder::new()
            .max_rect(egui::Rect::from_min_size(
                ui.cursor().min,
                egui::vec2(width, 0.),
            ))
            .layout(egui::Layout::top_down(egui::Align::Min)),
        |ui| {
            ui.set_width(width);
            ui.add(widget)
        },
    )
    .inner
}
