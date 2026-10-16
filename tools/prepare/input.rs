use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
pub(super) struct Input {
    pub id: String,
    pub file: String,
    pub sha256: String,
    pub name: String,
    pub url: String,
    pub year: String,
    pub license: String,
    pub normalization: String,
    pub columns: BTreeMap<String, String>,
    pub null_markers: Vec<String>,
    pub year_min: Option<f64>,
    pub year_max: Option<f64>,
}
