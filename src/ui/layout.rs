use super::{
    about,
    app::{Tab, TerraApp},
    header, map, ranking, results_summary, settings,
};
use bevy_egui::egui;

fn narrow(ui: &egui::Ui) -> bool {
    ui.ctx().content_rect().width() < 850.
}

pub(super) fn tab_selector(ui: &mut egui::Ui, tab: &mut usize, labels: &[&str]) {
    ui.allocate_ui_with_layout(
        egui::vec2(ui.available_width(), 44.),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.spacing_mut().button_padding.x = 4.;
            ui.spacing_mut().item_spacing.x = 4.;
            let width = (ui.available_width()
                - (labels.len() - 1) as f32 * ui.spacing().item_spacing.x)
                / labels.len() as f32;
            for (index, label) in labels.iter().enumerate() {
                let selected = *tab == index;
                let color = if selected {
                    ui.visuals().selection.stroke.color
                } else {
                    ui.visuals().text_color()
                };
                let icon = super::icons::tab(label, color, if width < 100. { 14. } else { 16. });
                let mut text = egui::RichText::new(*label);
                if width < 100. {
                    text = text.size(12.);
                }
                if ui
                    .add_sized(
                        [width, 44.],
                        egui::Button::selectable(selected, (icon, text)),
                    )
                    .clicked()
                {
                    *tab = index;
                }
            }
        },
    );
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
            .show(root, show);
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
        let tab = ui.ctx().data_mut(|data| {
            data.get_temp::<usize>(egui::Id::new("mobile_results_tab"))
                .unwrap_or(0)
        });
        let mut show = |ui: &mut egui::Ui| {
            if !results_summary::show(ui, &app.settings, &app.scores) {
                return;
            }
            if !narrow && ui.available_width() >= 1440. && ui.available_height() >= 720. {
                ui.columns(2, |columns| {
                    ranking::show(
                        &mut columns[0],
                        &app.data.countries,
                        &app.scores,
                        &mut app.selected,
                        app.settings.quality_weight(),
                    );
                    let map_height = (columns[1].available_height() - 40.).max(0.);
                    map::show(
                        &mut columns[1],
                        &app.data.countries,
                        &app.map,
                        &app.scores,
                        &mut app.selected,
                        map_height,
                    );
                });
                return;
            }
            if !narrow || tab == 1 {
                // Reserve most of the desktop height for the legend and ranking.
                let map_height = if narrow {
                    f32::INFINITY
                } else {
                    ui.available_height() * 0.45
                };
                map::show(
                    ui,
                    &app.data.countries,
                    &app.map,
                    &app.scores,
                    &mut app.selected,
                    map_height,
                );
            }
            if !narrow {
                ui.add_space(6.);
            }
            if !narrow || tab == 0 {
                ranking::show(
                    ui,
                    &app.data.countries,
                    &app.scores,
                    &mut app.selected,
                    app.settings.quality_weight(),
                );
            }
        };
        if narrow && tab == 1 {
            egui::ScrollArea::vertical()
                .id_salt(("results_scroll", narrow, tab))
                .auto_shrink([false, true])
                .show(ui, show);
        } else {
            show(ui);
        }
    });
}

#[cfg(test)]
#[path = "layout_test.rs"]
mod tests;
