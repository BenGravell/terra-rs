use crate::model::Settings;
use crate::ui::test_support::harness;
use bevy_egui::egui;
use egui_kittest::kittest::Queryable;

#[test]
fn settings_changes_update_results_and_reset_restores_defaults() {
    let mut h = harness(1400.);
    h.run();
    let original = h.state().scores[0].overall;
    let slider = h.get_by_label("Power Distance");
    slider.focus();
    h.key_press(egui::Key::ArrowRight);
    h.run();
    assert_eq!(h.state().settings.culture.power_distance, 60.);
    assert_ne!(h.state().scores[0].overall, original);
    h.get_by_label("Reset Culture").click_accesskit();
    h.run();
    assert_eq!(h.state().settings, Settings::default());
    assert_eq!(h.state().scores[0].overall, original);
}

#[test]
fn mobile_tabs_fill_bottom_row_and_switch_sections() {
    for (width, height) in [(320., 568.), (390., 844.), (844., 950.)] {
        let mut h = harness(width);
        h.set_size(egui::vec2(width, height));
        h.run();
        let first = h.get_by_label("Culture").rect();
        let last = h.get_by_label("Presets").rect();
        assert!(first.left() <= 10.);
        assert!(last.right() >= width - 10.);
        assert!(last.bottom() >= height - 10. && last.bottom() <= height);
        for label in ["Weights", "Filters", "Presets"] {
            let rect = h.get_by_label(label).rect();
            assert!(
                (rect.width() - first.width()).abs() <= 1.,
                "{label}: {rect:?}, first {first:?}, width {width}"
            );
            assert!((rect.bottom() - first.bottom()).abs() < 1.);
        }
        assert!(h.query_by_label("Power Distance").is_some());
        h.get_by_label("Weights").click();
        h.run();
        assert!(h.query_by_label("Power Distance").is_none());
        assert!(h.query_by_label("Culture Fit").is_some());
        h.get_by_label("Filters").click();
        h.run();
        assert!(h.query_by_label("Culture Fit Minimum").is_some());
        assert_eq!(h.get_by_label("Presets").rect(), last);
        h.get_by_label("Presets").click();
        h.run();
        assert!(h.query_by_label("Use Culture Preferences").is_some());
        h.get_by_label("Culture").click();
        h.run();
        assert!(h.query_by_label("Power Distance").is_some());
    }
}

#[test]
fn resets_only_change_their_own_section_on_mobile_and_desktop() {
    for width in [390., 1400.] {
        for section in ["Culture", "Weights", "Filters", "Presets"] {
            let mut h = harness(width);
            h.run();
            let mut changed = Settings::default();
            changed.culture.power_distance = 80.;
            changed.overall_importances.culture = 8.;
            changed.quality_importances.happy_planet = 2.;
            changed.minimums.culture = 0.3;
            changed.english_min = 0.2;
            changed.temperature.min = 5.;
            changed.sunshine.min = 5.;
            changed.continents = vec!["Europe".into()];
            h.state_mut().settings = changed.clone();
            h.state_mut().reference = Some(0);
            h.run();
            if width < 850. {
                h.get_by_label(section).click();
            } else if section == "Filters" {
                h.get_by_label("Filters").click_accesskit();
            } else if section == "Presets" {
                h.get_by_label("Presets").click_accesskit();
            }
            h.run();
            h.get_by_label(&format!("Reset {section}"))
                .click_accesskit();
            h.run();
            let defaults = Settings::default();
            match section {
                "Culture" => changed.culture = defaults.culture,
                "Weights" => {
                    changed.overall_importances = defaults.overall_importances;
                    changed.quality_importances = defaults.quality_importances;
                }
                "Filters" => {
                    changed.minimums = defaults.minimums;
                    changed.english_min = defaults.english_min;
                    changed.temperature = defaults.temperature;
                    changed.sunshine = defaults.sunshine;
                    changed.continents = defaults.continents;
                }
                _ => {}
            }
            assert_eq!(h.state().settings, changed, "{section} at {width}");
            assert_eq!(
                h.state().reference,
                if section == "Presets" { None } else { Some(0) }
            );
        }
    }
}

#[test]
fn local_reset_discards_focused_numeric_edits() {
    let mut h = harness(390.);
    h.run();
    h.state_mut().settings.culture.power_distance = 60.;
    h.state_mut().settings.overall_importances.culture = 8.;
    h.run();
    h.get_by_value("+1").click();
    h.run();
    h.key_press_modifiers(egui::Modifiers::COMMAND, egui::Key::A);
    h.event(egui::Event::Text("+4".into()));
    h.get_by_label("Reset Culture").click();
    h.run();
    assert_eq!(h.state().settings.culture, Settings::default().culture);
    assert_eq!(h.state().settings.overall_importances.culture, 8.);
}

#[test]
fn mobile_reset_is_full_width_and_pinned_above_tabs() {
    for (width, height) in [(320., 568.), (390., 844.)] {
        let mut h = harness(width);
        h.set_size(egui::vec2(width, height));
        h.run();
        for section in ["Culture", "Weights", "Filters", "Presets"] {
            h.get_by_label(section).click();
            h.run();
            let reset = h.get_by_label(&format!("Reset {section}")).rect();
            let first_tab = h.get_by_label("Culture").rect();
            let last_tab = h.get_by_label("Presets").rect();
            assert!((reset.left() - first_tab.left()).abs() < 1.);
            assert!((reset.right() - last_tab.right()).abs() < 1.);
            assert!((0. ..=24.).contains(&(first_tab.top() - reset.bottom())));
        }
    }
}

#[test]
fn desktop_sections_use_mobile_names_and_icons() {
    let mut h = harness(1400.);
    h.set_size(egui::vec2(1400., 1600.));
    h.run();
    for label in ["Culture", "Weights", "Filters", "Presets"] {
        let header = h.get_by_label(label).rect();
        assert!(
            h.output().shapes.iter().any(|shape| {
                matches!(&shape.shape, egui::Shape::Rect(image)
                if image.brush.is_some() && header.contains_rect(image.rect))
            }),
            "missing icon for {label}"
        );
    }
}
