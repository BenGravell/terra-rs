use super::{CONTINENTS, Country, Culture, Quality, Source};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Deserialize, Serialize)]
pub struct Dataset {
    pub schema_version: u32,
    pub sources: Vec<Source>,
    pub countries: Vec<Country>,
}

impl Dataset {
    pub fn parse(text: &str) -> Result<Self, String> {
        let data: Self = serde_json::from_str(text).map_err(|e| format!("Country data: {e}"))?;
        if data.schema_version != 1 {
            return Err("Unsupported country data version.".into());
        }
        let mut ids = HashSet::new();
        for c in &data.countries {
            if c.id.len() != 3
                || !c.id.bytes().all(|b| b.is_ascii_uppercase())
                || c.name.is_empty()
                || !ids.insert(&c.id)
            {
                return Err(format!("Invalid or duplicate country: {}", c.id));
            }
            let numbers = c
                .culture
                .iter()
                .flat_map(Culture::values)
                .chain(c.quality.iter().flat_map(Quality::values))
                .chain(c.english)
                .chain(c.temperature)
                .chain(c.sunshine);
            if numbers.into_iter().any(|x| !x.is_finite())
                || c.culture
                    .is_some_and(|v| v.values().iter().any(|x| *x < 0.))
                || c.quality
                    .is_some_and(|v| v.values().iter().any(|x| !(0.0..=1.0).contains(x)))
                || c.english.is_some_and(|v| !(0.0..=1.0).contains(&v))
                || c.continent
                    .as_ref()
                    .is_some_and(|v| !CONTINENTS.contains(&v.as_str()))
                || c.location.is_some_and(|location| {
                    !(-180.0..=180.0).contains(&location.longitude)
                        || !(-90.0..=90.0).contains(&location.latitude)
                })
            {
                return Err(format!("Invalid data values for {}", c.name));
            }
        }
        Ok(data)
    }
}

#[cfg(test)]
#[path = "dataset_test.rs"]
mod tests;
