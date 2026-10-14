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
        assert!(h.query_by_label(heading).is_some());
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
        settings.overall_weights.quality = quality_weight;
        settings.overall_weights.culture = 1. - quality_weight;
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
    assert_eq!(super::FLAG_PIXELS.len(), ids.len() * 32 * 24 * 4);
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
fn flag_edges_align_with_capital_letter_edges() {
    use bevy_egui::egui::{Shape, TextureId};
    let mut h = harness(1400.);
    h.run();
    let name = &h.state().data.countries[h.state().scores[1].country].name;
    let name_rect = h.get_by_label(name).rect();
    let text = h
        .output()
        .shapes
        .iter()
        .find_map(|shape| match &shape.shape {
            Shape::Text(text) if text.galley.job.text == *name => Some(text),
            _ => None,
        })
        .unwrap();
    let glyph = &text.galley.rows[0].glyphs[0];
    let capital_top = text.pos.y + glyph.pos.y + glyph.uv_rect.offset.y;
    let capital_bottom = capital_top + glyph.uv_rect.size.y;
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
    assert!((flag.top() - capital_top).abs() <= 1.);
    assert!((flag.bottom() - capital_bottom).abs() <= 1.);
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
            let source_ratio =
                uv.width() * 32. / (uv.height() * (super::FLAG_PIXELS.len() / (32 * 4)) as f32);
            assert!((rect.aspect_ratio() - source_ratio).abs() < 0.001);
            count += 1;
        }
    }
    assert!(count > 1);
}
