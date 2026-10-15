use crate::model::Settings;
use bevy_egui::egui;
use egui_kittest::{Harness, kittest::Queryable};

#[test]
fn importances_are_independent_and_only_the_last_nonzero_value_stops_at_one() {
    let mut h = Harness::builder()
        .with_size(egui::vec2(400., 800.))
        .build_ui_state(super::show, Settings::default());
    h.run();
    h.get_by_label("Culture Fit importance").focus();
    h.run();
    h.key_press(egui::Key::ArrowRight);
    h.run();
    assert_eq!(h.state().overall_importances.values(), [6., 5.]);
    for (label, expected) in [
        ("Culture Fit importance", [0., 5.]),
        ("Quality of Life importance", [0., 1.]),
    ] {
        h.get_by_label(label).focus();
        h.run();
        for _ in 0..12 {
            h.key_press(egui::Key::ArrowLeft);
            h.run();
            h.state().validate().unwrap();
        }
        assert_eq!(h.state().overall_importances.values(), expected);
    }
    for (label, expected) in [
        ("Happy Planet importance", [0., 4., 3.]),
        ("Social Progress importance", [0., 0., 3.]),
        ("Human Freedom importance", [0., 0., 1.]),
    ] {
        h.get_by_label(label).focus();
        h.run();
        for _ in 0..12 {
            h.key_press(egui::Key::ArrowLeft);
            h.run();
            h.state().validate().unwrap();
        }
        assert_eq!(h.state().quality_importances.values(), expected);
    }
    h.get_by_label("Happy Planet importance").focus();
    h.run();
    for _ in 0..12 {
        h.key_press(egui::Key::ArrowRight);
        h.run();
    }
    assert_eq!(h.state().quality_importances.values(), [10., 0., 1.]);
    h.get_by_label("Human Freedom importance").focus();
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
        ["Culture Fit importance", "Happy Planet importance"].map(|label| {
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
