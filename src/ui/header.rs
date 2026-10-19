use super::{app::Tab, theme};
use bevy_egui::egui;

pub(super) fn show(ui: &mut egui::Ui, narrow: bool, tab: &mut Tab) {
    ui.add_space(6.);
    if narrow {
        ui.spacing_mut().interact_size.y = 44.;
    }
    ui.horizontal(|ui| {
        theme::title(ui);
        if !narrow {
            if *tab == Tab::About {
                let mut blurb = egui::text::LayoutJob::default();
                let format = egui::TextFormat {
                    font_id: egui::TextStyle::Body.resolve(ui.style()),
                    color: ui.visuals().text_color(),
                    ..Default::default()
                };
                for (text, italics) in [
                    (
                        "is a country recommendation app designed to help you ",
                        false,
                    ),
                    ("find your place in the world", true),
                    (" by using data to answer ", false),
                    ("\"which country is right for me?\"", true),
                ] {
                    blurb.append(
                        text,
                        0.,
                        egui::TextFormat {
                            italics,
                            ..format.clone()
                        },
                    );
                }
                let text = blurb.text.clone();
                ui.allocate_ui_with_layout(
                    egui::vec2((ui.available_width() - 100.).max(0.), 25.),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| ui.add(egui::Label::new(blurb).truncate()),
                )
                .inner
                .on_hover_text(text);
            } else {
                ui.label("Find your place in the world");
            }
        }
        if narrow {
            let id = egui::Id::new("mobile_results_tab");
            let mut selected = ui
                .ctx()
                .data_mut(|data| data.get_temp::<usize>(id).unwrap_or(0));
            let open_id = egui::Id::new("mobile_results_open");
            let mut open = ui
                .ctx()
                .data_mut(|data| data.get_temp::<bool>(open_id).unwrap_or(true));
            let mut destination = if *tab == Tab::About {
                2
            } else if open {
                selected
            } else {
                usize::MAX
            };
            if let Some(clicked) =
                super::layout::tab_selector(ui, &mut destination, &["Table", "Map", "About"])
            {
                if clicked == 2 {
                    *tab = Tab::About;
                } else {
                    open = *tab == Tab::About || !open || clicked != selected;
                    *tab = Tab::Results;
                    selected = clicked;
                }
            }
            ui.ctx().data_mut(|data| {
                data.insert_temp(id, selected);
                data.insert_temp(open_id, open);
            });
            return;
        }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let (label, destination) = match tab {
                Tab::Results => ("About", Tab::About),
                Tab::About => ("Results", Tab::Results),
            };
            let icon = super::icons::tab(label, ui.visuals().text_color(), 16.);
            if ui.add(egui::Button::new((icon, label))).clicked() {
                *tab = destination;
            }
        });
    });
    ui.add_space(6.);
}

#[cfg(test)]
#[path = "header_test.rs"]
mod tests;
