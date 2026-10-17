use crate::model::Settings;
use bevy_egui::egui;
use egui_kittest::{
    Harness,
    kittest::{NodeT, Queryable},
};

#[test]
fn importances_are_independent_and_only_the_last_nonzero_value_stops_at_one() {
    let mut h = Harness::builder()
        .with_size(egui::vec2(400., 800.))
        .build_ui_state(
            |ui, state| {
                super::show(ui, state);
            },
            Settings::default(),
        );
    h.run();
    h.get_by_label("Culture Fit").focus();
    h.run();
    h.key_press(egui::Key::ArrowRight);
    h.run();
    assert_eq!(h.state().overall_importances.values(), [6., 5.]);
    for (label, expected) in [("Culture Fit", [0., 5.]), ("Quality of Life", [0., 1.])] {
        h.query_all_by_label(label)
            .find(|node| node.accesskit_node().numeric_value().is_some())
            .expect("importance slider")
            .focus();
        h.run();
        for _ in 0..12 {
            h.key_press(egui::Key::ArrowLeft);
            h.run();
            h.state().validate().unwrap();
        }
        assert_eq!(h.state().overall_importances.values(), expected);
    }
    for (label, expected) in [
        ("Happy Planet", [0., 5., 5.]),
        ("Social Progress", [0., 0., 5.]),
        ("Human Freedom", [0., 0., 1.]),
    ] {
        h.query_all_by_label(label)
            .find(|node| node.accesskit_node().numeric_value().is_some())
            .expect("importance slider")
            .focus();
        h.run();
        for _ in 0..12 {
            h.key_press(egui::Key::ArrowLeft);
            h.run();
            h.state().validate().unwrap();
        }
        assert_eq!(h.state().quality_importances.values(), expected);
    }
    h.get_by_label("Happy Planet").focus();
    h.run();
    for _ in 0..12 {
        h.key_press(egui::Key::ArrowRight);
        h.run();
    }
    assert_eq!(h.state().quality_importances.values(), [10., 0., 1.]);
    h.get_by_label("Human Freedom").focus();
    h.run();
    h.key_press(egui::Key::ArrowLeft);
    h.run();
    assert_eq!(h.state().quality_importances.values(), [10., 0., 0.]);
}

#[test]
fn knob_positions_stay_stable_when_peers_reach_zero() {
    let mut h = Harness::builder()
        .with_size(egui::vec2(400., 800.))
        .build_ui_state(
            |ui, settings| {
                ui.visuals_mut().handle_shape = egui::style::HandleShape::Circle;
                super::show(ui, settings);
            },
            Settings::default(),
        );
    h.run();
    let knobs = |h: &Harness<'_, Settings>| {
        ["Culture Fit", "Happy Planet"].map(|label| {
            let rect = h.get_by_label(label).rect();
            h.output()
                .shapes
                .iter()
                .find_map(|shape| {
                    if let egui::Shape::Circle(circle) = &shape.shape
                        && rect.contains(circle.center)
                    {
                        return Some(circle.center);
                    }
                    None
                })
                .expect("slider knob")
        })
    };
    let before = knobs(&h);
    h.state_mut().overall_importances.quality = 0.;
    h.state_mut().quality_importances.social_progress = 0.;
    h.state_mut().quality_importances.human_freedom = 0.;
    h.run();
    assert_eq!(knobs(&h), before);
    h.state_mut().overall_importances.quality = 5.;
    h.state_mut().quality_importances.social_progress = 4.;
    h.run();
    assert_eq!(knobs(&h), before);
}

#[test]
fn importance_help_opens_in_narrow_layout_without_changing_values() {
    let settings = Settings::default();
    let mut h = Harness::builder()
        .with_size(egui::vec2(320., 800.))
        .build_ui_state(
            |ui, state| {
                super::show(ui, state);
            },
            settings.clone(),
        );
    h.run();
    for (name, target) in [
        ("Culture Fit", "overall"),
        ("Quality of Life", "overall"),
        ("Happy Planet", "Quality of Life"),
        ("Social Progress", "Quality of Life"),
        ("Human Freedom", "Quality of Life"),
    ] {
        let help = super::importance_help(name, target);
        assert!(h.query_by_label(&help).is_none());
        let help_label = format!("Help: {name}");
        let button = h.get_by_label(&help_label);
        let rect = button.rect();
        assert!(rect.right() <= 320.);
        assert!(rect.width() < 32.);
        assert!((rect.width() - rect.height()).abs() < 1.);
        button.click();
        h.run();
        let popup = h.get_by_label(&help).rect();
        assert!(popup.left() >= 0. && popup.right() <= 320.);
        assert_eq!(h.state().overall_importances, settings.overall_importances);
        assert_eq!(h.state().quality_importances, settings.quality_importances);
        h.key_press(egui::Key::Escape);
        h.run();
        assert!(h.query_by_label(&help).is_none());
    }
}
