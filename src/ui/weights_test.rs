use crate::model::Settings;
use crate::ui::test_support::harness;
use bevy_egui::egui;
use egui_kittest::kittest::Queryable;

#[test]
fn weight_sliders_balance_peers_without_hiding_results_or_drifting() {
    let mut h = harness(1400.);
    h.run();
    h.get_by_label("Culture Fit weight").focus();
    h.run();
    h.key_press(egui::Key::ArrowRight);
    h.run();
    for (actual, expected) in h
        .state()
        .settings
        .overall_weights
        .values()
        .iter()
        .zip([0.51, 0.49])
    {
        assert!((actual - expected).abs() < 1e-12);
    }
    h.get_by_label("Happy Planet weight").focus();
    h.run();
    h.key_press(egui::Key::ArrowRight);
    h.run();
    for (actual, expected) in h
        .state()
        .settings
        .quality_weights
        .values()
        .iter()
        .zip([0.31, 0.40, 0.29])
    {
        assert!((actual - expected).abs() < 1e-12);
    }
    let settings = h.state().settings.clone();
    assert!(h.query_by_value("40%").is_some());
    assert!(h.query_by_value("29%").is_some());
    for _ in 0..10 {
        h.run();
        for readout in
            h.query_all_by(|node| node.numeric_value().is_some() && node.value().is_some())
        {
            assert!(!readout.value().unwrap().contains('.'), "{readout:?}");
        }
        assert_eq!(h.state().settings, settings);
        assert_eq!(h.state().scores.len(), 85);
        assert!(h.query_by_label("Check your settings").is_none());
        assert!(h.query_by_label("Total: 100% / 100%").is_none());
    }
    for _ in 0..100 {
        h.key_press(egui::Key::ArrowRight);
        h.run();
        h.state().settings.validate().unwrap();
        for weight in h.state().settings.quality_weights.values() {
            assert!((weight * 100. - (weight * 100.).round()).abs() < 1e-10);
        }
        assert_eq!(h.state().scores.len(), 85);
    }
    assert_eq!(
        h.state().settings.quality_weights,
        crate::model::Quality {
            happy_planet: 1.,
            social_progress: 0.,
            human_freedom: 0.
        }
    );
    h.get_by_label("Reset").click_accesskit();
    h.run();
    assert_eq!(h.state().settings, Settings::default());
}
