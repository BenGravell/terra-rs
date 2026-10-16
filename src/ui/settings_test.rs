use crate::model::Settings;
use crate::ui::test_support::harness;
use bevy_egui::egui;
use egui_kittest::kittest::Queryable;

#[test]
fn settings_changes_update_results_and_reset_restores_defaults() {
    let mut h = harness(1400.);
    h.run();
    let original = h.state().scores[0].overall;
    let slider = h.get_by_label("Power distance");
    slider.focus();
    h.key_press(egui::Key::ArrowRight);
    h.run();
    assert_eq!(h.state().settings.culture.power_distance, 60.);
    assert_ne!(h.state().scores[0].overall, original);
    h.get_by_label("Reset").click_accesskit();
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
        let last = h.get_by_label("Preset").rect();
        assert!(first.left() <= 10.);
        assert!(last.right() >= width - 10.);
        assert!(last.bottom() >= height - 10. && last.bottom() <= height);
        for label in ["Weights", "Limits", "Preset"] {
            let rect = h.get_by_label(label).rect();
            assert!(
                (rect.width() - first.width()).abs() <= 1.,
                "{label}: {rect:?}, first {first:?}, width {width}"
            );
            assert!((rect.bottom() - first.bottom()).abs() < 1.);
        }
        assert!(h.query_by_label("Power distance").is_some());
        h.get_by_label("Weights").click();
        h.run();
        assert!(h.query_by_label("Power distance").is_none());
        assert!(h.query_by_label("Culture Fit").is_some());
        h.get_by_label("Limits").click();
        h.run();
        assert!(h.query_by_label("Culture Fit minimum").is_some());
        assert_eq!(h.get_by_label("Preset").rect(), last);
        h.get_by_label("Preset").click();
        h.run();
        assert!(h.query_by_label("Use culture preferences").is_some());
        h.get_by_label("Culture").click();
        h.run();
        assert!(h.query_by_label("Power distance").is_some());
    }
}
