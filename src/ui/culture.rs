use crate::model::Culture;
use bevy_egui::egui;

pub(super) fn show(ui: &mut egui::Ui, culture: &mut Culture) {
    egui::CollapsingHeader::new("Culture fit")
        .default_open(true)
        .show(ui, |ui| {
            for (name, value, help) in [
                ("Power distance", &mut culture.power_distance, "Low: shared power. High: acceptance of hierarchy."),
                ("Individualism", &mut culture.individualism, "Low: strong group ties. High: individual independence."),
                ("Masculinity", &mut culture.masculinity, "Low: cooperation and care. High: competition and achievement."),
                ("Uncertainty avoidance", &mut culture.uncertainty_avoidance, "Low: comfort with ambiguity. High: preference for rules and predictability."),
                ("Long-term orientation", &mut culture.long_term_orientation, "Low: tradition and near-term outcomes. High: adaptation and long-term planning."),
                ("Indulgence", &mut culture.indulgence, "Low: restraint. High: freedom to enjoy life."),
            ] {
                let label = ui.label(name).on_hover_text(help);
                ui.add(
                    egui::Slider::new(value, 0.0..=100.0)
                        .step_by(1.)
                        .show_value(true),
                )
                .labelled_by(label.id)
                .on_hover_text(help);
            }
        });
}
