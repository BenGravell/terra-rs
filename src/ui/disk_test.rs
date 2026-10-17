use bevy_egui::egui;
use egui_kittest::{Harness, kittest::Queryable};

#[test]
fn numbers_and_question_marks_fit_centered_in_round_disks() {
    for text in ["?", "1", "42", "199"] {
        let mut h = Harness::builder().build_ui(|ui| {
            super::show(ui, text, 20., egui::Sense::click());
        });
        h.run();
        let disk = h.get_by_label(text).rect();
        assert!((disk.width() - 20.).abs() < 1.);
        assert!((disk.height() - 20.).abs() < 1.);
        let glyphs = h
            .output()
            .shapes
            .iter()
            .find_map(|shape| {
                if let egui::Shape::Text(text) = &shape.shape {
                    return Some(text.galley.rect.translate(text.pos.to_vec2()));
                }
                None
            })
            .unwrap();
        assert!(disk.contains_rect(glyphs));
        assert!((disk.center().x - glyphs.center().x).abs() < 1.);
        assert!((disk.center().y - glyphs.center().y).abs() < 1.);
    }
}
