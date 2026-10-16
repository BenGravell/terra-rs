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
        let label = super::help::label(ui, name, help);
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
