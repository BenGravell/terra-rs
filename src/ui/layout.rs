use super::{
    about,
    app::{Tab, TerraApp},
    header, map, ranking, results_summary, settings,
};
use bevy_egui::egui;

fn narrow(ui: &egui::Ui) -> bool {
    ui.ctx().content_rect().width() < 850.
}

pub(super) fn tab_selector(ui: &mut egui::Ui, tab: &mut usize, labels: &[&str]) -> Option<usize> {
    let mut clicked = None;
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
                    clicked = Some(index);
                }
            }
        },
    );
    clicked
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
        let id = egui::Id::new("mobile_settings_tab");
        let (mut tab, mut open) = root
            .ctx()
            .data_mut(|data| data.get_temp::<(usize, bool)>(id).unwrap_or((0, true)));
        egui::Panel::bottom("settings_tabs")
            .resizable(false)
            .show(root, |ui| {
                let mut selected = if open { tab } else { usize::MAX };
                if let Some(clicked) =
                    tab_selector(ui, &mut selected, &["Culture", "Weights", "Filters"])
                {
                    open = !open || clicked != tab;
                    tab = clicked;
                }
            });
        root.ctx()
            .data_mut(|data| data.insert_temp(id, (tab, open)));
        let results_open = root.ctx().data_mut(|data| {
            data.get_temp::<bool>(egui::Id::new("mobile_results_open"))
                .unwrap_or(true)
        });
        let available_height = root.available_height();
        let split_id = egui::Id::new("mobile_settings_split");
        let mut split = root
            .ctx()
            .data_mut(|data| data.get_temp::<f32>(split_id).unwrap_or(0.65));
        let drawer_height = if results_open {
            available_height * split
        } else {
            root.available_height()
        };
        egui::Panel::bottom("settings_narrow")
            .exact_size(drawer_height)
            .resizable(false)
            .show_collapsible(root, &mut open, |ui| {
                if results_open {
                    let (rect, response) = ui.allocate_exact_size(
                        egui::vec2(ui.available_width(), 8.),
                        egui::Sense::drag(),
                    );
                    let response = response.on_hover_cursor(egui::CursorIcon::ResizeVertical);
                    response.widget_info(|| {
                        egui::WidgetInfo::labeled(
                            egui::WidgetType::Slider,
                            ui.is_enabled(),
                            "Resize settings and results",
                        )
                    });
                    let keyboard_delta = if response.has_focus() {
                        ui.input(|input| {
                            (input.num_presses(egui::Key::ArrowDown) as f32
                                - input.num_presses(egui::Key::ArrowUp) as f32)
                                * 20.
                        })
                    } else {
                        0.
                    };
                    let delta = response.drag_delta().y + keyboard_delta;
                    if delta != 0. {
                        split = (split - delta / available_height.max(1.)).clamp(0.25, 0.8);
                        ui.ctx().data_mut(|data| data.insert_temp(split_id, split));
                        ui.ctx().request_repaint();
                    }
                    let visuals = ui.style().interact(&response);
                    ui.painter().rect_filled(
                        egui::Rect::from_center_size(rect.center(), egui::vec2(40., 4.)),
                        2.,
                        visuals.fg_stroke.color,
                    );
                }
                settings::mobile_content(
                    ui,
                    &app.data.countries,
                    &mut app.settings,
                    &mut app.reference,
                    &mut app.settings_revision,
                    app.message.as_deref(),
                    tab,
                );
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
    let show_results = |ui: &mut egui::Ui| {
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
        show(ui);
    };
    if narrow {
        let mut open = root.ctx().data_mut(|data| {
            data.get_temp::<bool>(egui::Id::new("mobile_results_open"))
                .unwrap_or(true)
        });
        egui::Panel::top("results_narrow")
            .exact_size(root.available_height())
            .resizable(false)
            .frame(egui::Frame::central_panel(root.style()))
            .show_collapsible(root, &mut open, show_results);
        let settings_open = root.ctx().data_mut(|data| {
            data.get_temp::<(usize, bool)>(egui::Id::new("mobile_settings_tab"))
                .unwrap_or((0, true))
                .1
        });
        egui::CentralPanel::default().show(root, |ui| {
            if !open && !settings_open {
                landing(ui);
            }
        });
    } else {
        egui::CentralPanel::default().show(root, show_results);
    }
}

fn landing(ui: &mut egui::Ui) {
    let width = (ui.available_width() - 56.).clamp(1., 648.);
    let font = egui::FontId::proportional(18.);
    let mut prompt = egui::text::LayoutJob::simple(
        "Adjust your Culture, Weights, and Filters settings and explore your results in the Table and Map.".into(),
        font,
        super::theme::TEXT,
        width,
    );
    prompt.halign = egui::Align::Center;
    for section in &mut prompt.sections {
        section.format.line_height = Some(28.8);
    }
    let prompt = ui.fonts_mut(|fonts| fonts.layout_job(prompt));
    let height = prompt.size().y;
    let rect = egui::Rect::from_center_size(ui.max_rect().center(), egui::vec2(width, height));
    ui.scope_builder(
        egui::UiBuilder::new()
            .max_rect(rect)
            .layout(egui::Layout::top_down(egui::Align::Center)),
        |ui| {
            ui.spacing_mut().item_spacing.y = 18.;
            ui.label(prompt);
        },
    );
}

#[cfg(test)]
#[path = "layout_test.rs"]
mod tests;
