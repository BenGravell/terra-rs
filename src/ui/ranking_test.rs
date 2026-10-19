use crate::ui::test_support::harness;
use egui_kittest::kittest::Queryable;

#[test]
fn ranking_selection_updates_the_selected_country() {
    let mut h = harness(1400.);
    h.run();
    let name = h.state().data.countries[h.state().scores[1].country]
        .name
        .clone();
    let first_name = &h.state().data.countries[h.state().scores[0].country].name;
    let first = h.get_by_label(first_name).rect();
    let second = h.get_by_label(&name).rect();
    assert!((first.left() - second.left()).abs() < 1.);
    assert!(second.top() > first.bottom());
    for heading in ["Rank", "Country", "Overall", "Score breakdown"] {
        assert!(h.query_by_label(heading).is_none());
    }
    h.get_by_label(&name).click();
    h.run();
    assert_eq!(h.state().selected, Some(h.state().scores[1].country));
    assert!(h.query_by_label("Country ranking").is_none());
}

#[test]
fn buildup_segments_sum_to_overall_with_zero_and_mixed_weights() {
    let data = crate::model::Dataset::parse(include_str!("../../assets/countries.json")).unwrap();
    for quality_weight in [0., 0.5, 1.] {
        let mut settings = crate::model::Settings::default();
        settings.overall_importances.quality = quality_weight * 10.;
        settings.overall_importances.culture = (1. - quality_weight) * 10.;
        for score in crate::scoring::evaluate(&data.countries, &settings).unwrap() {
            let parts = super::contributions(&score, quality_weight);
            assert!((parts.iter().sum::<f64>() - score.overall).abs() < 1e-12);
            assert!(
                (parts[1..].iter().sum::<f64>() - score.overall_contributions.quality).abs()
                    < 1e-12
            );
        }
    }
}

#[test]
fn bundled_flags_cover_every_rankable_country() {
    let ids: Vec<_> = super::FLAG_IDS.lines().collect();
    assert_eq!(super::FLAG_PIXELS.len(), ids.len() * 64 * 48 * 4);
    let data = crate::model::Dataset::parse(include_str!("../../assets/countries.json")).unwrap();
    let settings = crate::model::Settings::default();
    for country in data
        .countries
        .iter()
        .filter(|country| country.eligible(&settings))
    {
        assert!(
            ids.contains(&country.id.as_str()),
            "Missing flag for {}",
            country.id
        );
    }
}

#[test]
fn every_part_of_the_row_selects_the_country() {
    let mut h = harness(1400.);
    h.run();
    let country = h.state().scores[1].country;
    let name = h.state().data.countries[country].name.clone();
    let row = h.get_by_label(&format!("Select {name}")).rect();
    let name_rect = h.get_by_label(&name).rect();
    for x in [
        row.left() + 2.,
        name_rect.left() - 20.,
        name_rect.center().x,
        name_rect.right() + 8.,
        row.center().x,
        row.right() - 2.,
    ] {
        h.state_mut().selected = Some(h.state().scores[0].country);
        h.run();
        let pos = bevy_egui::egui::pos2(x, row.center().y);
        h.hover_at(pos);
        h.drag_at(pos);
        h.drop_at(pos);
        h.run();
        assert_eq!(
            h.state().selected,
            Some(country),
            "Click at {x} missed the row"
        );
    }
}

#[test]
fn flag_height_is_midway_between_rank_and_text() {
    use bevy_egui::egui::{Shape, TextureId};
    let mut h = harness(1400.);
    h.run();
    let name = &h.state().data.countries[h.state().scores[1].country].name;
    let name_rect = h.get_by_label(name).rect();
    let flag = h
        .output()
        .shapes
        .iter()
        .find_map(|shape| match &shape.shape {
            Shape::Mesh(mesh) if mesh.texture_id != TextureId::default() => {
                let rect = mesh.calc_bounds();
                (rect.right() < name_rect.left()
                    && rect.intersects(name_rect.expand(40.))
                    && (rect.center().y - name_rect.center().y).abs() < 8.)
                    .then_some(rect)
            }
            _ => None,
        })
        .unwrap();
    let rank = h.get_by_label("2").rect();
    let text = h
        .output()
        .shapes
        .iter()
        .find_map(|shape| match &shape.shape {
            Shape::Text(text) if text.galley.job.text == *name => Some(text),
            _ => None,
        })
        .unwrap();
    let capital_height = text.galley.rows[0].glyphs[0].uv_rect.size.y;
    let expected_height = (rank.height() + capital_height) * 0.5;
    assert!((flag.height() - expected_height).abs() <= 1.);
    assert!(flag.height() < rank.height() - 3.);
    assert!((flag.center().y - name_rect.center().y).abs() <= 1.);
}

#[test]
fn rendered_flags_preserve_the_source_aspect_ratio() {
    use bevy_egui::egui::{Rect, Shape, TextureId};
    let mut h = harness(1400.);
    h.run();
    let mut count = 0;
    let mut flag_left: Option<f32> = None;
    for shape in &h.output().shapes {
        if let Shape::Mesh(mesh) = &shape.shape {
            if mesh.texture_id == TextureId::default() {
                continue;
            }
            let uv = mesh.vertices.iter().fold(Rect::NOTHING, |rect, vertex| {
                rect.union(Rect::from_min_max(vertex.uv, vertex.uv))
            });
            let rect = mesh.calc_bounds();
            if let Some(left) = flag_left {
                assert!((rect.left() - left).abs() < 0.01);
            } else {
                flag_left = Some(rect.left());
            }
            let source_ratio = uv.width() * 256.
                / (uv.height() * (super::FLAG_IDS.lines().count().div_ceil(4) * 48) as f32);
            assert!((rect.aspect_ratio() - source_ratio).abs() < 0.001);
            count += 1;
        }
    }
    assert!(count > 1);
}

#[test]
fn stacked_bars_leave_only_a_thin_gap_at_row_edges() {
    use bevy_egui::egui::Shape;
    for width in [390., 1400.] {
        let mut h = harness(width);
        h.run();
        let name = &h.state().data.countries[h.state().scores[1].country].name;
        let row = h.get_by_label(&format!("Select {name}")).rect();
        let bar = h
            .output()
            .shapes
            .iter()
            .find_map(|shape| match &shape.shape {
                Shape::Rect(rect)
                    if rect.fill == super::CULTURE && row.contains(rect.rect.center()) =>
                {
                    Some(rect.rect)
                }
                _ => None,
            })
            .unwrap();
        assert!((bar.top() - row.top() - if width < 850. { 1. } else { 2. }).abs() < 0.1);
        assert!((row.bottom() - bar.bottom() - if width < 850. { 1. } else { 2. }).abs() < 0.1);
    }
}

#[test]
fn column_widths_do_not_change_with_filtered_results() {
    for width in [320., 390., 844., 1400.] {
        let mut h = harness(width);
        h.run();
        let columns = |h: &egui_kittest::Harness<'_, crate::ui::TerraApp>| {
            let row = h
                .query_all_by(|node| {
                    node.label()
                        .is_some_and(|label| label.starts_with("Select "))
                })
                .next()
                .unwrap()
                .rect();
            let score = h
                .output()
                .shapes
                .iter()
                .find_map(|shape| {
                    if let bevy_egui::egui::Shape::Rect(rect) = &shape.shape
                        && rect.fill == super::CULTURE
                        && row.contains(rect.rect.center())
                    {
                        return Some(rect.rect.left());
                    }
                    None
                })
                .unwrap();
            [row.left(), score, row.right()]
        };
        let original = columns(&h);
        for continent in ["Europe", "Oceania", "Asia"] {
            h.state_mut().settings.continents = vec![continent.into()];
            h.run();
            for (before, after) in original.into_iter().zip(columns(&h)) {
                assert!(
                    (before - after).abs() < 1.,
                    "column moved at {width}: {before} -> {after}"
                );
            }
        }
    }
}

#[test]
fn rapidly_changing_limits_do_not_draw_red_diagnostic_boxes() {
    use bevy_egui::egui::{Color32, Shape};
    for width in [390., 1400.] {
        let mut h = harness(width);
        h.run();
        for minimum in [0.5, 0.8, 0.2, 0.7, 0., 0.9, 0.4, 0.] {
            h.state_mut().settings.minimums.culture = minimum;
            h.step();
            assert!(
                !h.output().shapes.iter().any(|shape| {
                    matches!(&shape.shape, Shape::Rect(rect) if rect.stroke.color == Color32::RED)
                }),
                "red diagnostic at width {width}, limit {minimum}"
            );
        }
    }
}

#[test]
fn legend_stays_fixed_when_table_scrolls() {
    use bevy_egui::egui;
    for width in [390., 1400.] {
        let mut h = harness(width);
        h.run();
        let legend_rects = |h: &egui_kittest::Harness<'_, crate::ui::TerraApp>| {
            h.output()
                .shapes
                .iter()
                .filter_map(|shape| {
                    if let egui::Shape::Text(text) = &shape.shape
                        && text.galley.job.text.starts_with("■ ")
                    {
                        return Some(text.galley.rect.translate(text.pos.to_vec2()));
                    }
                    None
                })
                .collect::<Vec<_>>()
        };
        let before = legend_rects(&h);
        assert_eq!(before.len(), 4);
        let name = h.state().data.countries[h.state().scores[0].country]
            .name
            .clone();
        let label = format!("Select {name}");
        let row = h.get_by_label(&label).rect();
        h.event(egui::Event::PointerMoved(row.center()));
        h.event(egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            phase: egui::TouchPhase::Move,
            delta: egui::vec2(0., -200.),
            modifiers: egui::Modifiers::NONE,
        });
        h.run();
        assert!(h.get_by_label(&label).rect().top() < row.top());
        assert_eq!(legend_rects(&h), before);
    }
}
