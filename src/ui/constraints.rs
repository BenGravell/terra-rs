use crate::model::Settings;
use bevy_egui::egui;

pub(super) fn show(ui: &mut egui::Ui, settings: &mut Settings) -> bool {
    super::icons::section("Filters", false)
        .show(ui, |ui| {
            content(ui, settings);
            let clicked = super::settings::reset_button(ui, "Filters");
            if clicked {
                reset(settings);
            }
            clicked
        })
        .body_returned
        .unwrap_or(false)
}

pub(super) fn content(ui: &mut egui::Ui, settings: &mut Settings) {
    let minimums = &mut settings.minimums;
    ui.label("Overall Score");
    for (label, value) in [
        ("Culture Fit Minimum", &mut minimums.culture),
        ("Quality of Life Minimum", &mut minimums.quality),
    ] {
        percentage(ui, label, value);
    }
    ui.label("Quality of Life");
    for (label, value) in [
        ("Happy Planet Minimum", &mut minimums.happy_planet),
        ("Social Progress Minimum", &mut minimums.social_progress),
        ("Human Freedom Minimum", &mut minimums.human_freedom),
    ] {
        percentage(ui, label, value);
    }
    ui.label("Language");
    percentage(ui, "English-Speaking Minimum", &mut settings.english_min);
    ui.label("Annual Average Temperature (°C)");
    for (name, value) in [
        ("Minimum °C", &mut settings.temperature.min),
        ("Maximum °C", &mut settings.temperature.max),
    ] {
        super::slider::row(
            ui,
            name,
            "Annual average temperature limit in °C.",
            value,
            -10.0..=30.0,
            "",
            false,
        );
    }
    ui.label("Daily Sunshine (Hours)");
    for (name, value) in [
        ("Minimum Hours", &mut settings.sunshine.min),
        ("Maximum Hours", &mut settings.sunshine.max),
    ] {
        super::slider::row(
            ui,
            name,
            "Daily sunshine limit in hours.",
            value,
            3.0..=10.0,
            "",
            false,
        );
    }
    ui.label("Continents");
    super::continents::show(ui, &mut settings.continents);
}

fn percentage(ui: &mut egui::Ui, label: &str, value: &mut f64) {
    let mut pct = *value * 100.;
    if super::slider::row(
        ui,
        label,
        "The country must exceed this minimum percentage.",
        &mut pct,
        0.0..=100.0,
        "%",
        false,
    )
    .changed()
    {
        *value = pct / 100.;
    }
}

pub(super) fn reset(settings: &mut Settings) {
    let defaults = Settings::default();
    settings.minimums = defaults.minimums;
    settings.english_min = defaults.english_min;
    settings.temperature = defaults.temperature;
    settings.sunshine = defaults.sunshine;
    settings.continents = defaults.continents;
}
