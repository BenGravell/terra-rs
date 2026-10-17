use bevy_egui::egui;

pub(super) fn show(
    ui: &mut egui::Ui,
    text: &str,
    diameter: f32,
    sense: egui::Sense,
) -> egui::Response {
    ui.scope(|ui| {
        ui.spacing_mut().button_padding = egui::Vec2::ZERO;
        ui.spacing_mut().interact_size = egui::Vec2::splat(diameter);
        let mut font = egui::TextStyle::Body.resolve(ui.style());
        let width = ui
            .painter()
            .layout_no_wrap(text.into(), font.clone(), ui.visuals().text_color())
            .size()
            .x;
        font.size *= ((diameter - 4.) / width).min(1.);
        ui.add_sized(
            egui::Vec2::splat(diameter),
            egui::Button::new(egui::RichText::new(text).font(font))
                .corner_radius(diameter / 2.)
                .sense(sense),
        )
    })
    .inner
}

#[cfg(test)]
#[path = "disk_test.rs"]
mod tests;
