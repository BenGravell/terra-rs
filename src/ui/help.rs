use bevy_egui::egui;

pub(super) fn label(ui: &mut egui::Ui, name: &str, help: &str) -> egui::Response {
    let mobile = ui.ctx().content_rect().width() < 850.;
    let diameter = ui.text_style_height(&egui::TextStyle::Body) + 6.;
    ui.allocate_ui_with_layout(
        egui::vec2(
            if mobile { 155. } else { ui.available_width() },
            if mobile { 32. } else { diameter },
        ),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            let label = if mobile {
                ui.add_sized(
                    [134., diameter],
                    egui::Label::new(name).halign(egui::Align::Min).truncate(),
                )
            } else {
                ui.label(name)
            }
            .on_hover_text(help);
            let button = super::disk::show(ui, "?", diameter, egui::Sense::click());
            button.widget_info(|| {
                egui::WidgetInfo::labeled(egui::WidgetType::Button, true, format!("Help: {name}"))
            });
            egui::Popup::from_toggle_button_response(&button)
                .width(240.)
                .show(|ui| {
                    ui.label(help);
                });
            label
        },
    )
    .inner
}
