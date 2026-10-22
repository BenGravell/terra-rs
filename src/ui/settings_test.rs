use crate::model::Settings;
use crate::ui::test_support::harness;
use bevy_egui::egui;
use egui_kittest::kittest::{NodeT, Queryable};

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
        let last = h.get_by_label("Filters").rect();
        assert!(first.left() <= 10.);
        assert!(last.right() >= width - 10.);
        assert!(last.bottom() >= height - 10. && last.bottom() <= height);
        for label in ["Weights", "Filters"] {
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
        assert_eq!(h.get_by_label("Filters").rect(), last);
        assert!(h.query_by_value("Copy minimums from country…").is_some());
        h.get_by_label("Culture").click();
        h.run();
        assert!(h.query_by_label("Power Distance").is_some());
    }
}

#[test]
fn resets_only_change_their_own_section_on_mobile_and_desktop() {
    for width in [390., 1400.] {
        for section in ["Culture", "Weights", "Filters"] {
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
            h.state_mut().reference.culture = Some(0);
            h.state_mut().reference.quality = Some(1);
            h.run();
            if width < 850. {
                if section != "Culture" {
                    h.get_by_label(section).click();
                }
            } else if section == "Filters" {
                h.get_by_label("Filters").click_accesskit();
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
            assert_eq!(h.state().reference.culture, Some(0));
            assert_eq!(h.state().reference.quality, Some(1));
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
fn mobile_actions_share_a_row_and_stay_pinned_above_tabs() {
    for (width, height) in [(320., 568.), (390., 844.)] {
        let mut h = harness(width);
        h.set_size(egui::vec2(width, height));
        h.run();
        for section in ["Culture", "Weights", "Filters"] {
            if section != "Culture" {
                h.get_by_label(section).click();
            }
            h.run();
            let reset = h.get_by_label(&format!("Reset {section}")).rect();
            let first_tab = h.get_by_label("Culture").rect();
            let last_tab = h.get_by_label("Filters").rect();
            if section == "Weights" {
                assert!((reset.left() - first_tab.left()).abs() < 1.);
            } else {
                let prompt = if section == "Culture" {
                    "Copy culture from country…"
                } else {
                    "Copy minimums from country…"
                };
                let copy = h.get_by_value(prompt).rect();
                assert!((copy.left() - first_tab.left()).abs() < 1.);
                assert!(copy.right() < reset.left());
                assert!(copy.top() < reset.bottom() && reset.top() < copy.bottom());
            }
            assert!(
                (reset.right() - last_tab.right()).abs() < 1.,
                "{section} {width} reset {reset:?} tab {last_tab:?}"
            );
            assert!((0. ..=24.).contains(&(first_tab.top() - reset.bottom())));
        }
    }
}

#[test]
fn desktop_sections_use_mobile_names_and_icons() {
    let mut h = harness(1400.);
    h.set_size(egui::vec2(1400., 1600.));
    h.run();
    for label in ["Culture", "Weights", "Filters"] {
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

#[test]
fn mobile_settings_tabs_toggle_drawer() {
    let mut h = harness(390.);
    h.run();
    let tabs = h.get_by_label("Culture").rect();
    h.get_by_label("Culture").click();
    h.run();
    assert!(h.query_by_label("Power Distance").is_none());
    assert!(h.query_by_label("Reset Culture").is_none());
    assert_eq!(h.get_by_label("Culture").rect(), tabs);
    for section in ["Culture", "Weights", "Filters"] {
        h.get_by_label(section).click();
        h.run();
        assert!(h.query_by_label(&format!("Reset {section}")).is_some());
        h.get_by_label(section).click();
        h.run();
        assert!(h.query_by_label(&format!("Reset {section}")).is_none());
        assert_eq!(h.get_by_label("Culture").rect(), tabs);
    }
}

#[test]
fn country_shortcuts_apply_from_their_sections() {
    for width in [390., 1400.] {
        let mut h = harness(width);
        h.set_size(egui::vec2(width, 2400.));
        h.run();
        assert!(h.query_by_label("Presets").is_none());
        let index = h
            .state()
            .data
            .countries
            .iter()
            .position(|c| c.name == "Sweden")
            .unwrap();
        let country = h.state().data.countries[index].clone();
        let mut expected = Settings::default();
        expected.use_culture(&country);
        h.get_by_value("Copy culture from country…").click();
        h.run();
        h.event(egui::Event::Text("Sweden".into()));
        h.run();
        h.query_all_by_label("Sweden")
            .find(|node| node.accesskit_node().role() == egui::accesskit::Role::Button)
            .unwrap()
            .click();
        h.run();
        assert_eq!(h.state().settings, expected);
        h.get_by_label("Filters").click_accesskit();
        h.run();
        let germany = h
            .state()
            .data
            .countries
            .iter()
            .find(|c| c.name == "Germany")
            .unwrap()
            .clone();
        expected.use_quality_thresholds(&germany);
        h.get_by_value("Copy minimums from country…").click();
        h.run();
        h.event(egui::Event::Text("Germany".into()));
        h.run();
        h.query_all_by_label("Germany")
            .find(|node| node.accesskit_node().role() == egui::accesskit::Role::Button)
            .unwrap()
            .click();
        h.run();
        assert_eq!(h.state().settings, expected);
        if width >= 850. {
            h.get_by_label("Filters").click_accesskit();
            h.run();
        }
        assert!(h.query_by_value("Copy minimums from country…").is_some());
        if width < 850. {
            h.get_by_label("Culture").click();
            h.run();
        }
        assert!(h.query_by_value("Copy culture from country…").is_some());
    }
}
