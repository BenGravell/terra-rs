use super::*;
use crate::model::Settings;
use egui_kittest::{Harness, kittest::Queryable};

fn assert_shares(actual: Quality, expected: [f64; 3]) {
    for (actual, expected) in shares(actual).into_iter().zip(expected) {
        assert!((actual - expected).abs() < 1e-6, "{actual} != {expected}");
    }
    Settings {
        quality_importances: actual,
        ..Settings::default()
    }
    .validate()
    .unwrap();
}

#[test]
fn corners_center_edges_and_outside_drags_have_valid_shares() {
    let vertices = vertices(Rect::from_min_size(Pos2::ZERO, egui::vec2(300., 262.)));
    for expected in [
        [1., 0., 0.],
        [0., 1., 0.],
        [0., 0., 1.],
        [0., 0.5, 0.5],
        [0.2, 0.3, 0.5],
    ] {
        assert_shares(weights_at(vertices, position(vertices, expected)), expected);
    }
    for (point, expected) in [
        (vertices[0] - egui::vec2(0., 100.), [1., 0., 0.]),
        (vertices[1] - egui::vec2(100., 0.), [0., 1., 0.]),
        (vertices[2] + egui::vec2(100., 0.), [0., 0., 1.]),
        (
            vertices[1].lerp(vertices[2], 0.5) + egui::vec2(0., 100.),
            [0., 0.5, 0.5],
        ),
    ] {
        assert_shares(weights_at(vertices, point), expected);
    }
}

#[test]
fn off_grid_points_snap_to_nearest_ten_percent_shares() {
    let vertices = vertices(Rect::from_min_size(Pos2::ZERO, egui::vec2(300., 262.)));
    for (input, expected) in [
        ([0.23, 0.28, 0.49], [0.2, 0.3, 0.5]),
        ([0.02, 0.02, 0.96], [0., 0., 1.]),
        ([0.36, 0.33, 0.31], [0.4, 0.3, 0.3]),
    ] {
        assert_shares(weights_at(vertices, position(vertices, input)), expected);
    }
    for x in (-100..=400).step_by(13) {
        for y in (-100..=400).step_by(17) {
            let weights = weights_at(vertices, egui::pos2(x as f32, y as f32)).values();
            assert_eq!(weights.iter().sum::<f64>(), 10.);
            assert!(
                weights
                    .iter()
                    .all(|w| (0. ..=10.).contains(w) && w.fract() == 0.)
            );
        }
    }
}

#[test]
fn isolines_are_parallel_to_opposite_edges_and_keep_their_share() {
    let vertices = vertices(Rect::from_min_size(Pos2::ZERO, egui::vec2(300., 262.)));
    for i in 0..3 {
        let line = isoline(vertices, i, 0.3);
        let opposite = vertices[(i + 2) % 3] - vertices[(i + 1) % 3];
        assert!(((line[1] - line[0]) - opposite * 0.7).length() < 1e-4);
        for t in [0., 0.25, 0.5, 0.75, 1.] {
            assert!((shares(weights_at(vertices, line[0].lerp(line[1], t)))[i] - 0.3).abs() < 1e-6);
        }
    }
}

#[test]
fn pointer_keyboard_corners_and_reset_update_persistable_weights() {
    let mut h = Harness::builder()
        .with_size(egui::vec2(320., 800.))
        .build_ui_state(
            |ui, settings| {
                crate::ui::weights::content(ui, settings);
            },
            Settings::default(),
        );
    h.run();
    assert_eq!(h.state(), &Settings::default());
    let plot = h.get_by_label(LABEL).rect();
    assert!(plot.left() >= 0. && plot.right() <= 320.);
    let vertices = vertices(plot);
    assert!(plot.height() < 180.);
    for (index, name) in super::super::theme::QUALITY_NAMES
        .iter()
        .enumerate()
        .skip(1)
    {
        let label = h.get_by_label(name).rect();
        assert!((0. ..=12.).contains(&(vertices[index].y - label.bottom())));
        let side = vertices[0].lerp(
            vertices[index],
            (label.bottom() - vertices[0].y) / (vertices[index].y - vertices[0].y),
        );
        if index == 1 {
            assert!(label.right() < side.x);
        } else {
            assert!(label.left() > side.x);
        }
    }
    let target = position(vertices, [0.23, 0.28, 0.49]);
    h.hover_at(target);
    h.run();
    h.drag_at(target);
    h.run();
    h.drop_at(target);
    h.run();
    assert_shares(h.state().quality_importances, [0.2, 0.3, 0.5]);
    h.get_by_label(LABEL).focus();
    h.run();
    h.key_press(egui::Key::ArrowUp);
    h.run();
    assert!(shares(h.state().quality_importances)[0] > 0.2);
    assert!(
        h.state()
            .quality_importances
            .values()
            .iter()
            .all(|w| w.fract() == 0.)
    );
    let before = shares(h.state().quality_importances);
    h.key_press(egui::Key::ArrowRight);
    h.run();
    let after = shares(h.state().quality_importances);
    assert!((after[2] - before[2] - 0.1).abs() < 1e-6);
    h.key_press(egui::Key::Home);
    h.run();
    assert_shares(h.state().quality_importances, [1. / 3.; 3]);

    h.hover_at(position(vertices, [1. / 3.; 3]));
    h.run();
    h.drag_at(position(vertices, [1. / 3.; 3]));
    h.run();
    h.hover_at(vertices[2] + egui::vec2(50., 50.));
    h.run();
    assert_shares(h.state().quality_importances, [0., 0., 1.]);
    h.drop_at(vertices[2]);
    h.run();
    for (i, name) in super::super::theme::QUALITY_NAMES.iter().enumerate() {
        let corner = h.get_by_label(name);
        assert!(corner.rect().left() >= 0. && corner.rect().right() <= 320.);
        corner.click_accesskit();
        h.run();
        let mut expected = [0.; 3];
        expected[i] = 1.;
        assert_shares(h.state().quality_importances, expected);
    }
    let saved = serde_json::to_string(h.state()).unwrap();
    assert_eq!(&Settings::decode(&saved).unwrap(), h.state());
    assert_eq!(
        h.state().overall_importances,
        Settings::default().overall_importances
    );
    crate::ui::weights::reset(h.state_mut());
    h.run();
    assert_eq!(h.state(), &Settings::default());
}
