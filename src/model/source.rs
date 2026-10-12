use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Source {
    pub name: String,
    pub url: String,
    pub year: String,
}
