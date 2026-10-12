use serde_tuple::{Deserialize_tuple, Serialize_tuple};

/// Geographic longitude and latitude in degrees.
#[derive(Clone, Copy, Debug, Deserialize_tuple, Serialize_tuple, PartialEq)]
pub struct Location {
    pub longitude: f32,
    pub latitude: f32,
}

#[cfg(test)]
#[path = "location_test.rs"]
mod tests;
