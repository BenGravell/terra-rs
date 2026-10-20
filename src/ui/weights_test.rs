use crate::model::{Overall, Settings};
use bevy_egui::egui;
use egui_kittest::{
    Harness,
    kittest::{NodeT, Queryable},
};

#[test]
fn balance_keyboard_steps_reach_both_extremes_and_reset() {
    let mut h = Harness::builder()
        .with_size(egui::vec2(400., 800.))
        .build_ui_state(
            |ui, settings| {
                super::show(ui, settings);
            },
            Settings::default(),
        );
    h.run();
    h.get_by_label(super::BALANCE_LABEL).focus();
    h.run();
    h.key_press(egui::Key::ArrowRight);
    h.run();
    assert_eq!(h.state().overall_importances.values(), [4., 6.]);
    for (key, expected) in [
        (egui::Key::ArrowLeft, [10., 0.]),
        (egui::Key::ArrowRight, [0., 10.]),
    ] {
        for _ in 0..12 {
            h.key_press(key);
            h.run();
            h.state().validate().unwrap();
            assert_eq!(
                h.state().overall_importances.values().iter().sum::<f64>(),
                10.
            );
        }
        assert_eq!(h.state().overall_importances.values(), expected);
    }
    assert_eq!(
        h.state().quality_importances,
        Settings::default().quality_importances
    );
    h.get_by_label("Reset Weights").click_accesskit();
    h.run();
    assert_eq!(h.state(), &Settings::default());
}

#[test]
fn saved_overall_weights_keep_their_ratio_until_edited() {
    let settings = Settings {
        overall_importances: Overall {
            culture: 2.,
            quality: 5.,
        },
        ..Settings::default()
    };
    let mut h = Harness::builder()
        .with_size(egui::vec2(400., 800.))
        .build_ui_state(
            |ui, settings| {
                super::content(ui, settings);
            },
            settings.clone(),
        );
    h.run();
    assert_eq!(h.state(), &settings);
    let value = h
        .get_by_label(super::BALANCE_LABEL)
        .accesskit_node()
        .numeric_value()
        .unwrap();
    assert!((value - 50. / 7.).abs() < 1e-6);
    h.get_by_label(super::BALANCE_LABEL).focus();
    h.run();
    h.key_press(egui::Key::ArrowLeft);
    h.run();
    assert_eq!(h.state().overall_importances.values(), [4., 6.]);
    assert_eq!(
        Settings::decode(&serde_json::to_string(h.state()).unwrap()).unwrap(),
        *h.state(),
    );
}

#[test]
fn endpoint_annotations_fit_narrow_layouts_and_select_extremes() {
    for width in [320., 390., 1400.] {
        let mut h = Harness::builder()
            .with_size(egui::vec2(width, 900.))
            .build_ui_state(
                |ui, settings| {
                    super::content(ui, settings);
                },
                Settings::default(),
            );
        h.run();
        let track = h.get_by_label(super::BALANCE_LABEL).rect();
        assert!(track.left() >= 0. && track.right() <= width);
        for (name, expected, left) in [
            ("Culture Fit", [10., 0.], true),
            ("Quality of Life", [0., 10.], false),
        ] {
            let endpoint = h
                .query_all_by_label(name)
                .find(|node| node.accesskit_node().role() == egui::accesskit::Role::Button)
                .unwrap();
            let rect = endpoint.rect();
            assert!(rect.left() >= 0. && rect.right() <= width);
            assert!(if left {
                rect.right() <= track.center().x
            } else {
                rect.left() >= track.center().x
            });
            endpoint.click_accesskit();
            h.run();
            assert_eq!(h.state().overall_importances.values(), expected);
            h.state().validate().unwrap();
        }
    }
}
