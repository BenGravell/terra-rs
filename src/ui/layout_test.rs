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
        assert!(h.query_by_label("Reset Culture").is_some());
        assert!(h.query_by_label("Matches").is_none());
        assert!(h.query_by_label("Eligible countries").is_none());
        assert!(
            h.query_all_by(|node| node
                .label()
                .is_some_and(|label| label.starts_with("Select ")))
                .next()
                .is_some()
        );
        assert!(h.query_by_label("Full country ranking").is_none());
        let slider = h.get_by_label("Power Distance").rect();
        let results = h
            .query_all_by(|node| {
                node.label()
                    .is_some_and(|label| label.starts_with("Select "))
            })
            .next()
            .unwrap()
            .rect();
        assert!(results.top() > title.bottom());
        assert!(results.bottom() < slider.top());
        assert!(slider.bottom() < height);
        assert!(results.bottom() < height);
        let original = h.state().scores[0].overall;
        h.get_by_label("Power Distance").focus();
        h.key_press(egui::Key::ArrowRight);
        h.run();
        assert_eq!(h.state().settings.culture.power_distance, 60.);
        assert_ne!(h.state().scores[0].overall, original);
        let best = &h.state().scores[0];
        let name = h.state().data.countries[best.country].name.clone();
        assert!(h.query_by_label(&name).is_some());
        assert!(h.get_by_label(&name).rect().bottom() < height);
        assert!(h.query_by_label("Reset Culture").is_some());
    }
}

#[test]
fn narrow_layout_handles_empty_and_invalid_results() {
    let mut h = harness(390.);
    h.run();
    h.state_mut().settings.continents.clear();
    h.run();
    assert!(
        h.query_by_label("No countries match. Try relaxing the filters.")
            .is_some()
    );
    assert!(h.state().selected.is_none());
    h.state_mut().settings.overall_importances.culture = -1.;
    h.run();
    assert!(h.query_by_label("Check your settings").is_some());
    h.get_by_label("Weights").click();
    h.run();
    h.get_by_label("Reset Weights").click();
    h.run();
    h.get_by_label("Filters").click();
    h.run();
    h.get_by_label("Reset Filters").click();
    h.run();
    assert!(
        h.query_all_by(|node| node
            .label()
            .is_some_and(|label| label.starts_with("Select ")))
            .next()
            .is_some()
    );
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
        assert_eq!(h.query_by_label("About").is_some(), width < 850.);
        let results = h
            .get_by_label(if width < 850. { "About" } else { "Results" })
            .rect();
        assert!((results.right() - about.right()).abs() < 1.);
        assert!((results.center().y - about.center().y).abs() < 1.);
        assert!(h.query_by_label("Map: Natural Earth 5.1.2").is_some());
        assert!(h.query_by_label("Matches").is_none());
        h.get_by_label(if width < 850. { "Table" } else { "Results" })
            .click();
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
            assert!(h.query_by_label(heading).is_none());
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
                row.height() >= 18. && row.height() <= 26.,
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
        let results_right = if width >= 1800. {
            let map_label = h.get_by_label("Country map").rect();
            map_label.left() - 10.
        } else {
            width
        };
        let unused = results_right - row.right();
        assert!(
            (0. ..=24.).contains(&unused),
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
        let selected_label = "Country map";
        let table = h.get_by_label("Table").rect();
        let map = h.get_by_label("Map").rect();
        assert!((table.width() - map.width()).abs() < 1.);
        assert!(
            table.bottom()
                < h.query_all_by(|node| node
                    .label()
                    .is_some_and(|label| label.starts_with("Select ")))
                    .next()
                    .unwrap()
                    .rect()
                    .top()
        );
        assert!(h.query_by_label(selected_label).is_none());
        h.get_by_label("Map").click();
        h.run();
        assert!(
            h.query_all_by(|node| node
                .label()
                .is_some_and(|label| label.starts_with("Select ")))
                .next()
                .is_none()
        );
        assert!(h.query_by_label(selected_label).is_some());
        h.get_by_label("Weights").click();
        h.run();
        assert!(h.query_by_label(selected_label).is_some());
        h.get_by_label("Table").click();
        h.run();
        assert!(
            h.query_all_by(|node| node
                .label()
                .is_some_and(|label| label.starts_with("Select ")))
                .next()
                .is_some()
        );
        assert!(h.query_by_label(selected_label).is_none());
        assert_eq!(h.state().selected, selected);
    }
    let mut h = harness(1440.);
    h.run();
    assert!(h.query_by_label("Table").is_none());
    assert!(h.query_by_label("Map").is_none());
    assert!(
        h.query_all_by(|node| node
            .label()
            .is_some_and(|label| label.starts_with("Select ")))
            .next()
            .is_some()
    );
}

#[test]
fn about_intro_and_sources_fit_phone_and_desktop_widths() {
    for width in [320., 360., 390., 1440.] {
        let mut h = harness(width);
        h.run();
        h.get_by_label("About").click();
        h.run();
        let table_bottom = h.get_by_label("Map: Natural Earth 5.1.2").rect().bottom();
        h.run_steps(60);
        let settled_bottom = h.get_by_label("Map: Natural Earth 5.1.2").rect().bottom();
        assert!(
            (settled_bottom - table_bottom).abs() < 1.,
            "Table drifts between frames: {table_bottom} -> {settled_bottom}"
        );
        assert!(h.query_by_label("Start exploring").is_none());
        assert!(h.query_by_label("Overall score & ranking").is_some());
        let intro = "Terra is a country recommendation app designed to help you find your place in the world by using data to answer \"which country is right for me?\"";
        for label in [
            "About Terra",
            intro,
            "Name",
            "Year",
            "License",
            "Map: Natural Earth 5.1.2",
            "Public domain",
        ] {
            let rect = h.get_by_label(label).rect();
            assert!(
                rect.left() >= 0. && rect.right() <= width,
                "{label} overflows at {width}: {rect:?}"
            );
        }
        let header_bottom = h.get_by_label("Name").rect().bottom();
        let first_row_top = h.get_by_label(&h.state().data.sources[0].name).rect().top();
        assert!(
            (0. ..=16.).contains(&(first_row_top - header_bottom)),
            "Unexpected space before first row: {}",
            first_row_top - header_bottom
        );
        for source in &h.state().data.sources {
            let rect = h.get_by_label(&source.name).rect();
            assert!(rect.height() < 100., "Source row is too tall: {rect:?}");
            for label in [&source.year, &source.license] {
                assert!(
                    h.query_all_by_label(label)
                        .any(|node| { (node.rect().top() - rect.top()).abs() < 1. }),
                    "{label} is not aligned with {}",
                    source.name
                );
            }
            assert!(
                rect.left() >= 0. && rect.right() <= width,
                "{} overflows at {width}: {rect:?}",
                source.name
            );
        }
        h.get_by_label("Overall score & ranking").click();
        h.run();
        let explanation = h.get_by_label("The overall score combines Culture Fit and Quality of Life using your weights. Ranks number matching countries consecutively after constraints. Equal scores are ordered by country ID.").rect();
        assert!(explanation.left() >= 0. && explanation.right() <= width);
    }
}

#[test]
fn desktop_map_leaves_room_for_ranking_at_wide_and_short_sizes() {
    for (width, height) in [
        (850., 600.),
        (1366., 768.),
        (1440., 600.),
        (1600., 1080.),
        (2560., 720.),
    ] {
        let mut h = harness(width);
        h.set_size(egui::vec2(width, height));
        h.run();
        let selected_label = "Country map";
        let map_bottom = h.get_by_label(selected_label).rect().bottom();
        let heading = h
            .query_all_by(|node| {
                node.label()
                    .is_some_and(|label| label.starts_with("Select "))
            })
            .next()
            .unwrap()
            .rect();
        assert!(
            map_bottom < height * 0.65,
            "map crowds out table at {width} × {height}"
        );
        assert!(heading.top() > map_bottom);
        for score in h
            .state()
            .scores
            .iter()
            .filter(|score| score.matches)
            .take(3)
        {
            let name = &h.state().data.countries[score.country].name;
            let row = h.get_by_label(&format!("Select {name}")).rect();
            assert!(
                row.bottom() < height,
                "ranking hidden at {width} × {height}"
            );
        }
    }
}

#[test]
fn large_screens_place_map_beside_table_and_resize_back_to_stacked() {
    let mut h = harness(1920.);
    h.run();
    let selected = h.state().selected;
    let selected_label = "Country map";
    for (width, height, side_by_side) in [
        (1920., 1080., true),
        (2560., 1440., true),
        (1440., 1080., false),
        (2560., 720., false),
        (1920., 1080., true),
    ] {
        h.set_size(egui::vec2(width, height));
        h.run();
        let map_label = h.get_by_label(selected_label).rect();
        let table = h
            .query_all_by(|node| {
                node.label()
                    .is_some_and(|label| label.starts_with("Select "))
            })
            .next()
            .unwrap()
            .rect();
        if side_by_side {
            assert!(table.right() < map_label.left());
            assert!(table.top() < map_label.bottom());
        } else {
            assert!(table.top() > map_label.bottom());
        }
        assert!(table.bottom() < height);
        assert_eq!(h.state().selected, selected);
        let name = &h.state().data.countries[h.state().scores[0].country].name;
        let row = h.get_by_label(&format!("Select {name}")).rect();
        assert!(row.right() <= width && row.bottom() < height);
    }
}

#[test]
fn mobile_header_and_legend_stay_on_single_rows() {
    for width in [320., 360., 390., 600., 844.] {
        let mut h = harness(width);
        h.run();
        let title = h.get_by_label("TERRA").rect();
        for name in ["Table", "Map", "About"] {
            let button = h.get_by_label(name).rect();
            assert!((button.center().y - title.center().y).abs() < 1.);
            assert!(button.right() <= width, "{name} {button:?} at {width}");
        }
        let mut legends: Vec<_> = h
            .output()
            .shapes
            .iter()
            .filter_map(|shape| {
                if let egui::Shape::Text(text) = &shape.shape
                    && text.galley.job.text.starts_with("■ ")
                {
                    assert_eq!(text.galley.rows.len(), 1);
                    let rect = text.galley.rect.translate(text.pos.to_vec2());
                    assert!(rect.right() <= width - 8.);
                    return Some(rect);
                }
                None
            })
            .collect();
        assert_eq!(legends.len(), 4);
        legends.sort_by(|a, b| a.left().total_cmp(&b.left()));
        let bar_left = h
            .output()
            .shapes
            .iter()
            .find_map(|shape| {
                if let egui::Shape::Rect(rect) = &shape.shape
                    && rect.fill == crate::ui::theme::CULTURE
                {
                    return Some(rect.rect.left());
                }
                None
            })
            .unwrap();
        let row = h
            .query_all_by(|node| {
                node.label()
                    .is_some_and(|label| label.starts_with("Select "))
            })
            .next()
            .unwrap()
            .rect();
        if width <= 390. {
            assert!(legends[0].left() < bar_left);
        }
        assert!((legends.last().unwrap().right() - row.right()).abs() < 1.);
        assert!(legends.last().unwrap().right() <= row.right());
        assert!(
            legends
                .iter()
                .all(|rect| (rect.center().y - legends[0].center().y).abs() < 1.)
        );
    }
}

#[test]
fn ranking_fits_after_resizing_and_changing_results() {
    let mut h = harness(1440.);
    h.run();
    for width in [320., 844., 280., 390., 1440., 360.] {
        h.set_size(egui::vec2(width, 700.));
        h.state_mut().settings.minimums.culture = 0.;
        h.run();
        for row in h.query_all_by(|node| {
            node.label()
                .is_some_and(|label| label.starts_with("Select "))
        }) {
            assert!(
                row.rect().right() <= width - 8.,
                "row {:?} at {width}",
                row.rect()
            );
        }
        for shape in &h.output().shapes {
            if let egui::Shape::Rect(rect) = &shape.shape
                && rect.fill == crate::ui::theme::CULTURE
            {
                assert!(
                    rect.rect.right() <= shape.clip_rect.right(),
                    "bar exceeds its viewport at {width}"
                );
            }
        }
        h.state_mut().settings.minimums.culture = 0.7;
        h.run();
        for row in h.query_all_by(|node| {
            node.label()
                .is_some_and(|label| label.starts_with("Select "))
        }) {
            assert!(
                row.rect().right() <= width - 8.,
                "filtered row {:?} at {width}",
                row.rect()
            );
        }
    }
}

#[test]
fn mobile_table_fills_results_height() {
    for (width, height) in [(320., 568.), (390., 844.), (844., 1200.)] {
        let mut h = harness(width);
        h.set_size(egui::vec2(width, height));
        h.run();
        let settings_top = h.get_by_label("Power Distance").rect().top();
        let table_bottom = h
            .output()
            .shapes
            .iter()
            .find_map(|shape| {
                if let egui::Shape::Rect(rect) = &shape.shape
                    && rect.fill == crate::ui::theme::CULTURE
                {
                    return Some(shape.clip_rect.bottom());
                }
                None
            })
            .unwrap();
        assert!(
            table_bottom >= height * 0.25
                && table_bottom <= height * 0.5
                && table_bottom <= settings_top,
            "table bottom {table_bottom}, settings top {settings_top} at {width}x{height}"
        );
    }
}

#[test]
fn mobile_results_tabs_toggle_and_reopen_from_about() {
    for width in [320., 390., 844.] {
        let mut h = harness(width);
        h.run();
        let selected = h.state().selected;
        let selected_label = "Country map";
        let table_tab = h.get_by_label("Table").rect();
        for section in ["Table", "Map"] {
            if section == "Map" {
                h.get_by_label(section).click();
                h.run();
            }
            h.get_by_label(section).click();
            h.run();
            assert!(h.query_by_label(selected_label).is_none());
            assert!(
                h.query_all_by(|node| node
                    .label()
                    .is_some_and(|label| label.starts_with("Select ")))
                    .next()
                    .is_none()
            );
            assert!(h.query_by_label("Power Distance").is_some());
            assert_eq!(h.get_by_label("Table").rect(), table_tab);
            h.get_by_label(section).click();
            h.run();
            if section == "Table" {
                assert!(
                    h.query_all_by(|node| node
                        .label()
                        .is_some_and(|label| label.starts_with("Select ")))
                        .next()
                        .is_some()
                );
            } else {
                assert!(h.query_by_label(selected_label).is_some());
            }
        }
        h.get_by_label("Map").click();
        h.run();
        h.get_by_label("About").click();
        h.run();
        h.get_by_label("Map").click();
        h.run();
        assert!(h.query_by_label(selected_label).is_some());
        assert_eq!(h.state().selected, selected);
    }
}

#[test]
fn mobile_settings_content_stays_above_reset_when_results_are_closed() {
    for (width, height) in [(320., 568.), (390., 844.), (844., 950.)] {
        let mut h = harness(width);
        h.set_size(egui::vec2(width, height));
        h.run();
        let shared_top = h.get_by_label("Power Distance").rect().top();
        let tabs = h.get_by_label("Culture").rect();
        for section in ["Table", "Map"] {
            if section == "Map" {
                h.get_by_label(section).click();
                h.run();
            }
            h.get_by_label(section).click();
            h.run();
            let settings_top = h.get_by_label("Power Distance").rect().top();
            let header_bottom = h.get_by_label("Table").rect().bottom();
            assert!(settings_top >= header_bottom);
            let content_bottom = h.get_by_label("Indulgence").rect().bottom();
            let reset_top = h.get_by_label("Reset Culture").rect().top();
            assert!((0. ..=24.).contains(&(reset_top - content_bottom)));
            assert_eq!(h.get_by_label("Culture").rect(), tabs);
            h.get_by_label(section).click();
            h.run();
            assert!((h.get_by_label("Power Distance").rect().top() - shared_top).abs() < 1.);
        }
    }
}

#[test]
fn mobile_landing_is_centered_only_when_both_drawers_are_closed() {
    let prompt = "Adjust your Culture, Weights, and Filters settings and explore your results in the Table and Map.";
    let intro = "A country recommendation app designed to help you find your place in the world by using data to answer \"which country is right for me?\"";
    for (width, height) in [(320., 568.), (390., 844.), (844., 950.)] {
        let mut h = harness(width);
        h.set_size(egui::vec2(width, height));
        h.run();
        assert!(h.query_by_label(prompt).is_none());
        h.get_by_label("Table").click();
        h.run();
        assert!(h.query_by_label(prompt).is_none());
        h.get_by_label("Culture").click();
        h.run();
        assert!(h.query_by_label(intro).is_none());
        assert_eq!(h.query_all_by_label("TERRA").count(), 1);
        let cta = h.get_by_label(prompt).rect();
        let top = h.get_by_label("Table").rect().bottom() + 12.;
        let bottom = h.get_by_label("Culture").rect().top() - 6.;
        assert!((cta.center().x - width / 2.).abs() < 2.);
        assert!((cta.center().y - (top + bottom) / 2.).abs() < 12.);
        assert!(cta.top() > top && cta.bottom() < bottom);
        h.get_by_label("Weights").click();
        h.run();
        assert!(h.query_by_label(prompt).is_none());
        h.get_by_label("Weights").click();
        h.run();
        assert!(h.query_by_label(prompt).is_some());
        h.get_by_label("Map").click();
        h.run();
        assert!(h.query_by_label(prompt).is_none());
        h.get_by_label("About").click();
        h.run();
        assert!(h.query_by_label(prompt).is_none());
    }
}

#[test]
fn mobile_split_can_be_dragged_and_survives_tab_changes() {
    let mut h = harness(390.);
    h.run();
    let label = "Resize settings and results";
    let start = h.get_by_label(label).rect().center();
    let target = start + egui::vec2(0., 80.);
    h.hover_at(start);
    h.run();
    h.drag_at(start);
    h.run();
    h.hover_at(target);
    h.run();
    h.drop_at(target);
    h.run();
    let moved = h.get_by_label(label).rect().center().y;
    assert!((moved - start.y - 80.).abs() < 2.);
    h.get_by_label("Map").click();
    h.run();
    assert!((h.get_by_label(label).rect().center().y - moved).abs() < 2.);
    h.get_by_label("Map").click();
    h.run();
    assert!(h.query_by_label(label).is_none());
    h.get_by_label("Map").click();
    h.run();
    assert!((h.get_by_label(label).rect().center().y - moved).abs() < 2.);
}
