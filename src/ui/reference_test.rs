#[test]
fn every_reference_country_has_a_flag() {
    let data = crate::model::Dataset::parse(include_str!("../../assets/countries.json")).unwrap();
    for country in data
        .countries
        .iter()
        .filter(|c| c.culture.is_some() && c.quality.is_some())
    {
        assert!(
            super::super::flags::FLAG_IDS
                .lines()
                .any(|id| id == country.id),
            "Missing flag for {}",
            country.name
        );
    }
}

#[test]
fn reference_search_filters_selects_and_clears_on_reopen() {
    use bevy_egui::egui;
    use egui_kittest::{Harness, kittest::Queryable};
    for width in [320., 600.] {
        let data =
            crate::model::Dataset::parse(include_str!("../../assets/countries.json")).unwrap();
        let sweden = data
            .countries
            .iter()
            .position(|country| country.name == "Sweden")
            .unwrap();
        let mut h = Harness::builder()
            .with_size(egui::vec2(width, 600.))
            .build_ui_state(
                move |ui, state: &mut (Option<usize>, crate::model::Settings, u64)| {
                    crate::ui::theme::apply(ui.ctx());
                    if super::content(
                        ui,
                        &data.countries,
                        &mut state.0,
                        "Copy culture from country…",
                    ) {
                        state.1.use_culture(&data.countries[state.0.unwrap()]);
                        state.2 += 1;
                    }
                },
                (None, crate::model::Settings::default(), 0),
            );
        h.run();
        h.get_by_value("Copy culture from country…").click();
        h.run();
        h.event(egui::Event::Text("sWeD".into()));
        h.run();
        assert!(h.query_by_label("Sweden").is_some());
        assert!(h.query_by_label("Germany").is_none());
        h.get_by_label("Sweden").click();
        h.run();
        assert_eq!(h.state().0, Some(sweden));
        assert_eq!(h.state().2, 1);
        h.get_by_value("Copy culture from country…").click();
        h.run();
        assert!(h.query_by_label("Germany").is_some());
        h.event(egui::Event::Text("zzzzzz".into()));
        h.run();
        assert!(h.query_by_label("No countries found.").is_some());
        h.key_press(egui::Key::Escape);
        h.run();
        assert!(h.query_by_label("No countries found.").is_none());
        assert_eq!(h.state().0, Some(sweden));
        assert_eq!(h.state().2, 1);
        h.get_by_value("Copy culture from country…").click();
        h.run();
        h.event(egui::Event::Text("Sweden".into()));
        h.run();
        h.get_by_label("Sweden").click();
        h.run();
        assert_eq!(h.state().2, 2, "the same country can be reapplied");
    }
}
