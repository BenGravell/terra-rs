use crate::ui::{test_support::harness, theme::score_color};
use bevy_egui::egui::Shape;

#[test]
fn title_gradient_runs_from_teal_to_blue() {
    let mut h = harness(1440.);
    h.run();
    let title = h
        .output()
        .shapes
        .iter()
        .find_map(|shape| match &shape.shape {
            Shape::Text(text) if text.galley.job.text == "TERRA" => Some(text),
            _ => None,
        })
        .unwrap();
    let vertices = &title.galley.rows[0].visuals.mesh.vertices;
    let left = vertices
        .iter()
        .min_by(|a, b| a.pos.x.total_cmp(&b.pos.x))
        .unwrap();
    let right = vertices
        .iter()
        .max_by(|a, b| a.pos.x.total_cmp(&b.pos.x))
        .unwrap();
    assert_eq!(left.color, score_color(0.45));
    assert_eq!(right.color, score_color(0.80));
    assert_ne!(left.color, right.color);
}

#[test]
fn about_introduction_appears_only_in_the_header() {
    use egui_kittest::kittest::Queryable;
    let mut h = harness(1440.);
    h.run();
    h.get_by_label("About").click();
    h.run();
    let title = h.get_by_label("TERRA").rect();
    let intro = h.get_by_label("is a country recommendation app designed to help you find your place in the world by using data to answer \"which country is right for me?\"").rect();
    assert!(intro.left() > title.right());
    assert!((intro.center().y - title.center().y).abs() < 2.);
    assert!(h.query_by_label("Find your place in the world").is_none());
}

#[test]
fn about_tagline_starts_immediately_after_title_on_wide_screens() {
    use egui_kittest::kittest::Queryable;
    for width in [1440., 1920., 2560.] {
        let mut h = harness(width);
        h.run();
        h.get_by_label("About").click();
        h.run();
        let title = h.get_by_label("TERRA").rect();
        let intro = h
            .output()
            .shapes
            .iter()
            .find_map(|shape| match &shape.shape {
                Shape::Text(text)
                    if text
                        .galley
                        .job
                        .text
                        .starts_with("is a country recommendation") =>
                {
                    Some(text)
                }
                _ => None,
            })
            .unwrap();
        let gap = intro.pos.x + intro.galley.rect.left() - title.right();
        assert!(
            (9. ..=11.).contains(&gap),
            "tagline gap {gap} at width {width}"
        );
    }
}
