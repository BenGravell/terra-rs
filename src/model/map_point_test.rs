use super::MapPoint;
use serde_json::json;

#[test]
fn map_coordinates_preserve_x_and_y() {
    let point: MapPoint = serde_json::from_str("[0.25,0.75]").unwrap();
    assert_eq!(point, MapPoint { x: 0.25, y: 0.75 });
    assert_eq!(point.values(), [0.25, 0.75]);
    assert_eq!(serde_json::to_value(point).unwrap(), json!([0.25, 0.75]));
}
