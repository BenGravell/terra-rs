use arraygen::Arraygen;
use serde_tuple::{Deserialize_tuple, Serialize_tuple};

/// Hofstede dimensions used for country data and user preferences.
#[derive(Arraygen, Clone, Copy, Debug, Deserialize_tuple, Serialize_tuple, PartialEq)]
#[gen_array(pub fn values: f64, implicit_select_all: _)]
pub struct Culture {
    pub power_distance: f64,
    pub individualism: f64,
    pub masculinity: f64,
    pub uncertainty_avoidance: f64,
    pub long_term_orientation: f64,
    pub indulgence: f64,
}
