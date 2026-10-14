use super::*;

#[test]
fn score_gradient_preserves_endpoints_and_colorful_midpoint() {
    assert_eq!(score_color(-1.), Color32::from_rgb(180, 35, 73));
    assert_eq!(score_color(0.), Color32::from_rgb(180, 35, 73));
    assert_eq!(score_color(0.5), Color32::from_rgb(171, 67, 190));
    assert_eq!(score_color(1.), Color32::from_rgb(30, 99, 233));
    assert_eq!(score_color(2.), Color32::from_rgb(30, 99, 233));
}
