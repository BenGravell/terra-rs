use arraygen::Arraygen;
use serde_tuple::{Deserialize_tuple, Serialize_tuple};

/// Quality-of-life components, used for scores, weights, and contributions.
#[derive(Arraygen, Clone, Copy, Debug, Deserialize_tuple, Serialize_tuple, PartialEq)]
#[gen_array(pub fn values: f64, implicit_select_all: _)]
pub struct Quality {
    pub happy_planet: f64,
    pub social_progress: f64,
    pub human_freedom: f64,
}
