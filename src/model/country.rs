use super::{Culture, EPS, Location, Quality, Settings};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Country {
    pub id: String,
    pub name: String,
    pub culture: Option<Culture>,
    pub quality: Option<Quality>,
    pub english: Option<f64>,
    pub temperature: Option<f64>,
    pub sunshine: Option<f64>,
    pub continent: Option<String>,
    pub location: Option<Location>,
}

impl Country {
    pub fn eligible(&self, settings: &Settings) -> bool {
        self.culture.is_some()
            && self.quality.is_some()
            && self.temperature.is_some()
            && self.sunshine.is_some()
            && self.continent.is_some()
            && (settings.english_min <= EPS || self.english.is_some())
    }
}
