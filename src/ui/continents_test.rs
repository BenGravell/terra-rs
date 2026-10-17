use bevy_egui::egui;
use egui_kittest::{Harness, kittest::Queryable};

#[test]
fn chips_fit_two_columns_and_toggle_independently() {
    for width in [280., 320., 390.] {
        let mut h = Harness::builder()
            .with_size(egui::vec2(width, 300.))
            .build_ui_state(super::show, crate::model::Settings::default().continents);
        h.run();
        for (index, name) in crate::model::CONTINENTS.into_iter().enumerate() {
            let rect = h.get_by_label(name).rect();
            assert!(rect.right() <= width - 8.);
            let first = h
                .get_by_label(crate::model::CONTINENTS[index / 2 * 2])
                .rect();
            assert_eq!(rect.top(), first.top());
            if index % 2 == 1 {
                assert!(rect.left() > first.right());
            }
            h.get_by_label(name).click();
            h.run();
            assert!(!h.state().iter().any(|value| value == name));
            assert_eq!(h.state().len(), 5);
            h.get_by_label(name).click();
            h.run();
            assert_eq!(h.state().len(), 6);
        }
    }
}

#[test]
fn geographic_outlines_are_closed_and_fit_the_icon_square() {
    for rings in super::OUTLINES {
        assert!(!rings.is_empty());
        assert!(rings[0].len() >= 8);
        for ring in rings {
            assert_eq!(ring.first(), ring.last());
            let area = ring
                .windows(2)
                .map(|edge| edge[0][0] * edge[1][1] - edge[1][0] * edge[0][1])
                .sum::<f32>()
                .abs()
                / 2.;
            assert!(area >= 2.9, "island too small to render clearly: {area}");
            assert!(
                ring.iter()
                    .flatten()
                    .all(|coordinate| coordinate.is_finite() && (0. ..=32.).contains(coordinate))
            );
        }
    }
    // Preserve island shapes for Africa, Europe and Oceania.
    for index in [0, 2, 3] {
        assert!(super::OUTLINES[index].len() > 1);
    }
}
