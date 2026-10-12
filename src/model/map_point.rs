use arraygen::Arraygen;
use serde_tuple::{Deserialize_tuple, Serialize_tuple};

/// Equirectangular coordinates in [0, 1], with y increasing southward.
#[derive(Arraygen, Clone, Copy, Debug, Deserialize_tuple, Serialize_tuple, PartialEq)]
#[gen_array(pub fn values: f32, implicit_select_all: _)]
pub struct MapPoint {
    pub x: f32,
    pub y: f32,
}

#[cfg(test)]
#[path = "map_point_test.rs"]
mod tests;
