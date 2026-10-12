use super::Location;
use serde_json::json;

#[test]
fn geographic_coordinates_preserve_longitude_and_latitude() {
    let location: Location = serde_json::from_str("[120.0,-30.0]").unwrap();
    assert_eq!(
        location,
        Location {
            longitude: 120.,
            latitude: -30.
        }
    );
    assert_eq!(serde_json::to_value(location).unwrap(), json!([120., -30.]));
}
