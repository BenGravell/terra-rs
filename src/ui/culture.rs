use crate::model::Culture;
use bevy_egui::egui;

pub(super) fn show(ui: &mut egui::Ui, culture: &mut Culture) {
    egui::CollapsingHeader::new("Culture fit")
        .default_open(true)
        .show(ui, |ui| {
            content(ui, culture);
        });
}

pub(super) fn content(ui: &mut egui::Ui, culture: &mut Culture) {
    let diameter = ui.text_style_height(&egui::TextStyle::Body) + 6.;
    for (name, value, help) in [
        (
            "Power distance",
            &mut culture.power_distance,
            "Low: shared power. High: acceptance of hierarchy.",
        ),
        (
            "Individualism",
            &mut culture.individualism,
            "Low: strong group ties. High: individual independence.",
        ),
        (
            "Masculinity",
            &mut culture.masculinity,
            "Low: cooperation and care. High: competition and achievement.",
        ),
        (
            "Uncertainty avoidance",
            &mut culture.uncertainty_avoidance,
            "Low: comfort with ambiguity. High: preference for rules and predictability.",
        ),
        (
            "Long-term orientation",
            &mut culture.long_term_orientation,
            "Low: tradition and near-term outcomes. High: adaptation and long-term planning.",
        ),
        (
            "Indulgence",
            &mut culture.indulgence,
            "Low: restraint. High: freedom to enjoy life.",
        ),
    ] {
        let label = ui
            .allocate_ui_with_layout(
                egui::vec2(ui.available_width(), diameter),
                egui::Layout::left_to_right(egui::Align::Center),
                |ui| {
                    let label = ui.label(name).on_hover_text(help);
                    ui.scope(|ui| {
                        ui.spacing_mut().button_padding = egui::Vec2::ZERO;
                        ui.spacing_mut().interact_size = egui::Vec2::splat(diameter);
                        let button = ui.add(
                            egui::Button::new(
                                egui::RichText::new("?").text_style(egui::TextStyle::Body),
                            )
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
            .inner;
        let mut importance = (*value / 10.).round() - 5.;
        if super::slider::show(ui, &mut importance, -5.0..=5.0, label.id, "", true)
            .on_hover_text(help)
            .changed()
        {
            *value = (importance + 5.) * 10.;
        }
    }
}

#[cfg(test)]
#[path = "culture_test.rs"]
mod tests;
