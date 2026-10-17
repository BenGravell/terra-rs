use bevy_egui::egui;
use egui_kittest::{Harness, kittest::Queryable};

#[test]
fn slider_fills_width_and_keeps_value_box_fixed_when_editing() {
    for width in [280., 390., 700.] {
        let mut h = Harness::builder()
            .with_size(egui::vec2(width, 200.))
            .build_ui_state(
                |ui, value| {
                    crate::ui::theme::apply(ui.ctx());
                    let label = ui.label("Value");
                    super::show(ui, value, 0.0..=100.0, label.id, "%", false);
                },
                9.,
            );
        h.run();
        let before = h.get_by_value("9%").rect();
        let track = h.get_by_label("Value").rect();
        assert!(
            (track.center().y - before.center().y).abs() < 1.,
            "track={track:?} value={before:?}"
        );
        assert!((before.width() - 40.).abs() < 1.);
        assert!((before.right() - (width - 8.)).abs() < 1.);
        h.get_by_value("9%").click();
        h.run();
        h.key_press_modifiers(egui::Modifiers::COMMAND, egui::Key::A);
        h.event(egui::Event::Text("73.6".into()));
        h.key_press(egui::Key::Enter);
        h.run();
        assert_eq!(*h.state(), 74.);
        let after = h.get_by_value("74%").rect();
        assert_eq!(before, after);
        h.get_by_label("Value").focus();
        h.key_press(egui::Key::ArrowRight);
        h.run();
        assert_eq!(*h.state(), 75.);
    }
}
