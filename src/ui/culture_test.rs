use bevy_egui::egui;
use egui_kittest::{Harness, kittest::Queryable};

#[test]
fn culture_help_opens_on_click_in_narrow_layout() {
    let culture = crate::model::Settings::default().culture;
    let mut h = Harness::builder()
        .with_size(egui::vec2(320., 800.))
        .build_ui_state(super::show, culture);
    h.run();

    for (name, help) in [
        (
            "Power distance",
            "Low: shared power. High: acceptance of hierarchy.",
        ),
        (
            "Individualism",
            "Low: strong group ties. High: individual independence.",
        ),
        (
            "Masculinity",
            "Low: cooperation and care. High: competition and achievement.",
        ),
        (
            "Uncertainty avoidance",
            "Low: comfort with ambiguity. High: preference for rules and predictability.",
        ),
        (
            "Long-term orientation",
            "Low: tradition and near-term outcomes. High: adaptation and long-term planning.",
        ),
        ("Indulgence", "Low: restraint. High: freedom to enjoy life."),
    ] {
        assert!(h.query_by_label(help).is_none());
        let help_label = format!("Help: {name}");
        let button = h.get_by_label(&help_label);
        assert!(button.rect().right() <= 320.);
        button.click();
        h.run();
        let text = h.get_by_label(help).rect();
        assert!(text.left() >= 0. && text.right() <= 320.);
        assert_eq!(*h.state(), culture);
        h.key_press(egui::Key::Escape);
        h.run();
        assert!(h.query_by_label(help).is_none());
    }
}

#[test]
fn help_buttons_are_small_and_beside_labels() {
    for width in [320., 390., 700.] {
        let mut h = Harness::builder()
            .with_size(egui::vec2(width, 800.))
            .build_ui_state(
                |ui, culture| {
                    crate::ui::theme::apply(ui.ctx());
                    super::show(ui, culture);
                },
                crate::model::Settings::default().culture,
            );
        h.run();
        let values: Vec<_> = h
            .query_all_by(|node| node.numeric_value().is_some() && node.value().is_some())
            .map(|node| node.rect())
            .collect();
        assert_eq!(values.len(), 6);
        for (name, value) in [
            "Power distance",
            "Individualism",
            "Masculinity",
            "Uncertainty avoidance",
            "Long-term orientation",
            "Indulgence",
        ]
        .into_iter()
        .zip(values)
        {
            let button = h.get_by_label(&format!("Help: {name}")).rect();
            assert!(button.width() < 32.);
            assert!((button.width() - button.height()).abs() < 1.);
            assert!(button.right() < value.center().x);
            assert!((value.width() - crate::ui::slider::VALUE_WIDTH).abs() < 1.);
            assert!((value.right() - (width - 8.)).abs() < 1.);
        }
    }
}

#[test]
fn culture_sliders_use_integer_minus_five_to_five_values() {
    let mut h = Harness::builder()
        .with_size(egui::vec2(400., 800.))
        .build_ui_state(super::show, crate::model::Settings::default().culture);
    h.run();
    assert_eq!(h.query_all_by_value("0").count(), 6);
    h.get_by_label("Power distance").focus();
    h.run();
    h.key_press(egui::Key::ArrowRight);
    h.run();
    assert_eq!(h.state().power_distance, 60.);
    assert!(h.query_by_value("+1").is_some());
    for _ in 0..12 {
        h.key_press(egui::Key::ArrowRight);
        h.run();
    }
    assert_eq!(h.state().power_distance, 100.);
    assert!(h.query_by_value("+5").is_some());
    for _ in 0..12 {
        h.key_press(egui::Key::ArrowLeft);
        h.run();
    }
    assert_eq!(h.state().power_distance, 0.);
    assert!(h.query_by_value("-5").is_some());
    h.get_by_value("-5").click();
    h.run();
    h.key_press_modifiers(egui::Modifiers::COMMAND, egui::Key::A);
    h.event(egui::Event::Text("+3".into()));
    h.key_press(egui::Key::Enter);
    h.run();
    assert_eq!(h.state().power_distance, 80.);
    assert!(h.query_by_value("+3").is_some());
}
