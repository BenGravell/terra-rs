use super::{
    about,
    app::{Tab, TerraApp},
    header, map, ranking, results_summary, settings,
};
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

pub(super) fn header(root: &mut egui::Ui, tab: &mut Tab) {
    let narrow = narrow(root);
    egui::Panel::top("header").show(root, |ui| header::show(ui, narrow, tab));
}

pub(super) fn startup_error(root: &mut egui::Ui, error: &str) {
    egui::CentralPanel::default().show(root, |ui| {
        ui.heading("Terra could not load its data");
        ui.label(error);
    });
}

pub(super) fn settings(root: &mut egui::Ui, app: &mut TerraApp) {
    if app.tab == Tab::About {
        return;
    }
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
    } else {
        let height = (root.available_height() * 0.48).min(420.);
        egui::Panel::bottom("settings_narrow")
            .exact_size(height)
            .resizable(false)
            .show(root, |ui| {
                egui::ScrollArea::vertical()
                    .id_salt("settings_narrow_scroll")
                    .auto_shrink([false, false])
                    .show(ui, show);
            });
    }
}

pub(super) fn results(root: &mut egui::Ui, app: &mut TerraApp) {
    if app.tab == Tab::About {
        egui::CentralPanel::default().show(root, |ui| {
            egui::ScrollArea::vertical()
                .id_salt("about_scroll")
                .show(ui, |ui| about::show(ui, &app.data.sources));
        });
        return;
    }
    let narrow = narrow(root);
    egui::CentralPanel::default().show(root, |ui| {
        egui::ScrollArea::vertical()
            .id_salt("results_scroll")
            .auto_shrink([false, true])
            .show(ui, |ui| {
                if !results_summary::show(ui, &app.settings, &app.scores) {
                    return;
                }
                if narrow {
                    ranking::show(
                        ui,
                        &app.data.countries,
                        &app.scores,
                        &mut app.selected,
                        app.settings.quality_weight(),
                    );
                    egui::CollapsingHeader::new("World map").show(ui, |ui| {
                        map::show(
                            ui,
                            &app.data.countries,
                            &app.map,
                            &app.scores,
                            &mut app.selected,
                        );
                    });
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
                ranking::show(
                    ui,
                    &app.data.countries,
                    &app.scores,
                    &mut app.selected,
                    app.settings.quality_weight(),
                );
            });
    });
}

#[cfg(test)]
#[path = "layout_test.rs"]
mod tests;
