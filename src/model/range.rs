use arraygen::Arraygen;
use serde_tuple::{Deserialize_tuple, Serialize_tuple};

/// Lower and upper bounds for a climate constraint.
#[derive(Arraygen, Clone, Copy, Debug, Deserialize_tuple, Serialize_tuple, PartialEq)]
#[gen_array(pub fn values: f64, implicit_select_all: _)]
pub struct Range {
    pub min: f64,
    pub max: f64,
}
