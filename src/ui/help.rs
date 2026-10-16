use bevy_egui::egui;

pub(super) fn label(ui: &mut egui::Ui, name: &str, help: &str) -> egui::Response {
    let diameter = ui.text_style_height(&egui::TextStyle::Body) + 6.;
    ui.allocate_ui_with_layout(
        egui::vec2(ui.available_width(), diameter),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            let label = ui.label(name).on_hover_text(help);
            ui.scope(|ui| {
                ui.spacing_mut().button_padding = egui::Vec2::ZERO;
                ui.spacing_mut().interact_size = egui::Vec2::splat(diameter);
                let button = ui.add(
                    egui::Button::new(egui::RichText::new("?").text_style(egui::TextStyle::Body))
                        .corner_radius(diameter / 2.)
                        .min_size(egui::Vec2::splat(diameter)),
                );
                button.widget_info(|| {
                    egui::WidgetInfo::labeled(
                        egui::WidgetType::Button,
                        true,
                        format!("Help: {name}"),
                    )
                });
                egui::Popup::from_toggle_button_response(&button)
                    .width(240.)
                    .show(|ui| {
                        ui.label(help);
                    });
            });
            label
        },
    )
    .inner
}
