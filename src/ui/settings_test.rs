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
