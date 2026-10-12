use arraygen::Arraygen;
use serde_tuple::{Deserialize_tuple, Serialize_tuple};

/// Overall score components, used for weights and contributions.
#[derive(Arraygen, Clone, Copy, Debug, Deserialize_tuple, Serialize_tuple, PartialEq)]
#[gen_array(pub fn values: f64, implicit_select_all: _)]
pub struct Overall {
    pub culture: f64,
    pub quality: f64,
}
