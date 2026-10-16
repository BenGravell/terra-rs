use crate::ui::test_support::harness;
use bevy_egui::egui;
use egui_kittest::kittest::Queryable;

#[test]
fn narrow_layout_places_live_results_above_settings() {
    for (width, height) in [(320., 568.), (390., 844.), (844., 950.)] {
        let mut h = harness(width);
        h.set_size(egui::vec2(width, height));
        h.run();
        assert!(h.query_by_label("Settings").is_none());
        assert!(h.query_by_label("Results").is_none());
        let title = h.get_by_label("TERRA").rect();
        let about = h.get_by_label("About").rect();
        assert!((title.center().y - about.center().y).abs() < 1.);
        assert!(about.right() > width - 20. && about.right() <= width);
        assert!(h.query_by_label("Reset").is_some());
        assert!(h.query_by_label("Matches").is_none());
        assert!(h.query_by_label("Eligible countries").is_none());
        assert!(h.query_by_label("Country").is_some());
        assert!(h.query_by_label("Full country ranking").is_none());
        let slider = h.get_by_label("Power distance").rect();
        let results = h.get_by_label("Country").rect();
        assert!(results.top() > title.bottom());
        assert!(results.bottom() < slider.top());
        assert!(slider.bottom() < height);
        assert!(results.bottom() < height);
        let original = h.state().scores[0].overall;
        h.get_by_label("Power distance").focus();
        h.key_press(egui::Key::ArrowRight);
        h.run();
        assert_eq!(h.state().settings.culture.power_distance, 60.);
        assert_ne!(h.state().scores[0].overall, original);
        let best = &h.state().scores[0];
        let name = h.state().data.countries[best.country].name.clone();
        assert!(h.query_by_label(&name).is_some());
        assert!(h.get_by_label(&name).rect().bottom() < height);
        assert!(h.query_by_label("Reset").is_some());
    }
}

#[test]
fn narrow_layout_handles_empty_and_invalid_results() {
    let mut h = harness(390.);
    h.run();
    h.state_mut().settings.continents.clear();
    h.run();
    assert!(
        h.query_by_label("No countries match. Try lowering a minimum or allowing more continents.")
            .is_some()
    );
    assert!(h.state().selected.is_none());
    h.state_mut().settings.overall_importances.culture = -1.;
    h.run();
    assert!(h.query_by_label("Check your settings").is_some());
    h.get_by_label("Reset").click();
    h.run();
    assert!(h.query_by_label("Country").is_some());
    assert!(h.state().selected.is_some());
}

#[test]
fn top_right_button_switches_pages_on_wide_and_narrow_layouts() {
    for width in [320., 390., 1440.] {
        let mut h = harness(width);
        h.run();
        assert!(h.query_by_label("Results").is_none());
        let about = h.get_by_label("About").rect();
        let title = h.get_by_label("TERRA").rect();
        assert!((title.center().y - about.center().y).abs() < 1.);
        assert!(about.right() > width - 20. && about.right() <= width);
        assert!(h.query_by_label("Map: Natural Earth 5.1.2").is_none());
        h.get_by_label("About").click();
        h.run();
        assert!(h.state().tab == super::Tab::About);
        assert!(h.query_by_label("About").is_none());
        let results = h.get_by_label("Results").rect();
        assert!((results.right() - about.right()).abs() < 1.);
        assert!((results.center().y - about.center().y).abs() < 1.);
        assert!(h.query_by_label("Map: Natural Earth 5.1.2").is_some());
        assert!(h.query_by_label("Matches").is_none());
        h.get_by_label("Results").click();
        h.run();
        assert!(h.state().tab == super::Tab::Results);
        assert!(h.query_by_label("Results").is_none());
        assert!(h.query_by_label("About").is_some());
        assert!(h.query_by_label("Matches").is_none());
    }
}

#[test]
fn phone_ranking_rows_fit_without_horizontal_scrolling() {
    for width in [320., 360., 390., 430.] {
        let mut h = harness(width);
        h.run();
        for heading in ["#", "Country", "Score", "Breakdown"] {
            assert!(h.get_by_label(heading).rect().right() <= width - 8.);
        }
        assert!(h.query_by_label("Top matches").is_none());
        assert!(h.query_by_label("Full country ranking").is_none());
        let names: Vec<_> = h
            .state()
            .scores
            .iter()
            .filter(|score| score.matches)
            .map(|score| h.state().data.countries[score.country].name.clone())
            .collect();
        for name in &names {
            let row = h.get_by_label(&format!("Select {name}")).rect();
            assert!(
                row.right() <= width - 8.,
                "{name} extends to {} at width {width}",
                row.right()
            );
            assert!(
                row.height() >= 28.,
                "{name} height {} at width {width}",
                row.height()
            );
        }
        let selected = h.state().scores[1].country;
        h.get_by_label(&format!(
            "Select {}",
            h.state().data.countries[selected].name
        ))
        .click();
        h.run();
        assert_eq!(h.state().selected, Some(selected));
    }
}

#[test]
fn ranking_uses_the_full_results_width() {
    for width in [320., 390., 600., 844., 1400., 2560.] {
        let mut h = harness(width);
        h.run();
        let name = &h.state().data.countries[h.state().scores[0].country].name;
        let row = h.get_by_label(&format!("Select {name}")).rect();
        let unused = width - row.right();
        assert!(
            (8. ..=24.).contains(&unused),
            "Unused width {unused} at page width {width}"
        );
    }
}

#[test]
fn mobile_results_tabs_switch_views_and_preserve_selection() {
    for width in [320., 390., 844.] {
        let mut h = harness(width);
        h.run();
        let selected = h.state().selected;
        let selected_label = format!(
            "Selected: {}",
            h.state().data.countries[selected.unwrap()].name
        );
        let table = h.get_by_label("Table").rect();
        let map = h.get_by_label("Map").rect();
        assert!((table.width() - map.width()).abs() < 1.);
        assert!(table.bottom() < h.get_by_label("Country").rect().top());
        assert!(h.query_by_label(&selected_label).is_none());
        h.get_by_label("Map").click();
        h.run();
        assert!(h.query_by_label("Country").is_none());
        assert!(h.query_by_label(&selected_label).is_some());
        h.get_by_label("Weights").click();
        h.run();
        assert!(h.query_by_label(&selected_label).is_some());
        h.get_by_label("Table").click();
        h.run();
        assert!(h.query_by_label("Country").is_some());
        assert!(h.query_by_label(&selected_label).is_none());
        assert_eq!(h.state().selected, selected);
    }
    let mut h = harness(1440.);
    h.run();
    assert!(h.query_by_label("Table").is_none());
    assert!(h.query_by_label("Map").is_none());
    assert!(h.query_by_label("Country").is_some());
}

#[test]
fn about_topics_expand_within_phone_and_desktop_widths() {
    for width in [320., 390., 1440.] {
        let mut h = harness(width);
        h.run();
        h.get_by_label("About").click();
        h.run();
        let heading = h.get_by_label("Which country feels like home?").rect();
        assert!(heading.left() >= 0. && heading.right() <= width);
        h.get_by_label("Overall score & ranking").click();
        h.run();
        let explanation = h.get_by_label("The overall score combines Culture Fit and Quality of Life using your weights. Ranks number matching countries consecutively after constraints. Equal scores are ordered by country ID.").rect();
        assert!(explanation.left() >= 0. && explanation.right() <= width);
        assert!(explanation.width() <= 800.);
    }
}
