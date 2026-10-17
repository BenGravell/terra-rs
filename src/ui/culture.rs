use crate::model::Culture;
use bevy_egui::egui;

pub(super) fn show(ui: &mut egui::Ui, culture: &mut Culture) -> bool {
    super::icons::section("Culture", true)
        .show(ui, |ui| {
            content(ui, culture);
            let clicked = super::settings::reset_button(ui, "Culture");
            if clicked {
                reset(culture);
            }
            clicked
        })
        .body_returned
        .unwrap_or(false)
}

pub(super) fn content(ui: &mut egui::Ui, culture: &mut Culture) {
    for (name, value, help) in [
        (
            "Power Distance",
            &mut culture.power_distance,
            "Negative: shared power. Positive: acceptance of hierarchy.",
        ),
        (
            "Individualism",
            &mut culture.individualism,
            "Negative: strong group ties. Positive: individual independence.",
        ),
        (
            "Masculinity",
            &mut culture.masculinity,
            "Negative: cooperation and care. Positive: competition and achievement.",
        ),
        (
            "Uncertainty Avoidance",
            &mut culture.uncertainty_avoidance,
            "Negative: comfort with ambiguity. Positive: preference for rules and predictability.",
        ),
        (
            "Long-Term Orientation",
            &mut culture.long_term_orientation,
            "Negative: tradition and near-term outcomes. Positive: adaptation and long-term planning.",
        ),
        (
            "Indulgence",
            &mut culture.indulgence,
            "Negative: restraint. Positive: freedom to enjoy life.",
        ),
    ] {
        let mut importance = (*value / 10.).round() - 5.;
        if super::slider::row(ui, name, help, &mut importance, -5.0..=5.0, "", true)
            .on_hover_text(help)
            .changed()
        {
            *value = (importance + 5.) * 10.;
        }
    }
}

pub(super) fn reset(culture: &mut Culture) {
    *culture = crate::model::Settings::default().culture;
}

#[cfg(test)]
#[path = "culture_test.rs"]
mod tests;
