use crate::ui::test_support::harness;
use egui_kittest::kittest::Queryable;

#[test]
fn invalid_weights_hide_results_and_recovery_restores_them() {
    let mut h = harness(1400.);
    h.run();
    h.state_mut().settings.overall_importances = crate::model::Overall {
        culture: 0.,
        quality: 0.,
    };
    h.run();
    assert!(h.state().scores.is_empty());
    assert!(h.query_by_label("Check your settings").is_some());
    h.get_by_label("Reset").click();
    h.run();
    assert_eq!(h.state().scores.len(), 85);
}
