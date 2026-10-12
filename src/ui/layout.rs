use super::{about, app::TerraApp, bars, header, map, ranking, results_summary, settings};
use bevy_egui::egui;

fn narrow(ui: &egui::Ui) -> bool {
    ui.ctx().content_rect().width() < 850.
}

pub(super) fn root(ctx: &egui::Context) -> egui::Ui {
    egui::Ui::new(
        ctx.clone(),
        "terra_root".into(),
        egui::UiBuilder::new().max_rect(ctx.content_rect()),
    )
}

pub(super) fn header(root: &mut egui::Ui, settings_tab: &mut bool) {
    let narrow = narrow(root);
    egui::Panel::top("header").show(root, |ui| header::show(ui, narrow, settings_tab));
}

pub(super) fn startup_error(root: &mut egui::Ui, error: &str) {
    egui::CentralPanel::default().show(root, |ui| {
        ui.heading("Terra could not load its data");
        ui.label(error);
    });
}

pub(super) fn settings(root: &mut egui::Ui, app: &mut TerraApp) {
    let show = |ui: &mut egui::Ui| {
        settings::show(
            ui,
            &app.data.countries,
            &mut app.settings,
            &mut app.reference,
            &mut app.settings_revision,
            app.message.as_deref(),
        )
    };
    if !narrow(root) {
        egui::Panel::left("settings")
            .exact_size(330.)
            .resizable(false)
            .show(root, |ui| {
                egui::ScrollArea::vertical()
                    .id_salt("settings_scroll")
                    .show(ui, show);
            });
    } else if app.settings_tab {
        egui::CentralPanel::default().show(root, |ui| {
            egui::ScrollArea::vertical()
                .id_salt("settings_narrow")
                .show(ui, show);
        });
    }
}

pub(super) fn results(root: &mut egui::Ui, app: &mut TerraApp) {
    if narrow(root) && app.settings_tab {
        return;
    }
    egui::CentralPanel::default().show(root, |ui| {
        egui::ScrollArea::vertical()
            .id_salt("results_scroll")
            .show(ui, |ui| {
                if !results_summary::show(ui, &app.settings, &app.scores) {
                    return;
                }
                map::show(
                    ui,
                    &app.data.countries,
                    &app.map,
                    &app.scores,
                    &mut app.selected,
                );
                ui.add_space(6.);
                ranking_and_bars(ui, app);
                ui.add_space(14.);
                about::show(ui, &app.data.sources);
            });
    });
}

fn ranking_and_bars(ui: &mut egui::Ui, app: &mut TerraApp) {
    if ui.available_width() > 850. {
        ui.columns(2, |cols| {
            ranking::show(
                &mut cols[0],
                &app.data.countries,
                &app.scores,
                &mut app.selected,
            );
            bars::show(
                &mut cols[1],
                &app.data.countries,
                &app.scores,
                &mut app.selected,
                &mut app.bar_count,
            );
        });
    } else {
        ranking::show(ui, &app.data.countries, &app.scores, &mut app.selected);
        ui.separator();
        bars::show(
            ui,
            &app.data.countries,
            &app.scores,
            &mut app.selected,
            &mut app.bar_count,
        );
    }
}

#[cfg(test)]
#[path = "layout_test.rs"]
mod tests;
