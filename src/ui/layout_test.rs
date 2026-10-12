use crate::ui::test_support::harness;
use egui_kittest::kittest::Queryable;

#[test]
fn narrow_layout_switches_between_settings_and_results() {
    let mut h = harness(390.);
    h.run();
    h.get_by_label("Settings").click();
    h.run();
    assert!(h.state().settings_tab);
    assert!(h.query_by_label("Reset").is_some());
    h.get_by_label("Results").click();
    h.run();
    assert!(!h.state().settings_tab);
    assert!(h.query_by_label("Your world, ranked").is_some());
}
