use arraygen::Arraygen;
use serde_tuple::{Deserialize_tuple, Serialize_tuple};

/// Strict lower score thresholds; values at or below EPS are inactive.
#[derive(Arraygen, Clone, Copy, Debug, Deserialize_tuple, Serialize_tuple, PartialEq)]
#[gen_array(pub fn values: f64, implicit_select_all: _)]
pub struct Minimums {
    pub culture: f64,
    pub quality: f64,
    pub happy_planet: f64,
    pub social_progress: f64,
    pub human_freedom: f64,
}
