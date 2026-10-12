use super::Result;
use serde::Serialize;
use std::collections::BTreeMap;
use terra::model::{Country, MapRegion, Settings};

#[derive(Serialize)]
pub(super) struct Coverage {
    pub eligible: Vec<String>,
    excluded: BTreeMap<String, Vec<&'static str>>,
    marker_only: Vec<String>,
}

impl Coverage {
    pub(super) fn build(countries: &[Country], map: &[MapRegion]) -> Result<Self> {
        let mut excluded = BTreeMap::new();
        for c in countries {
            let name = &c.name;
            let mut reasons = Vec::new();
            if c.culture.is_none() {
                reasons.push("culture");
            }
            if c.quality.is_none() {
                reasons.push("quality-of-life");
            }
            if c.temperature.is_none() {
                reasons.push("temperature");
            }
            if c.sunshine.is_none() {
                reasons.push("sunshine");
            }
            if c.continent.is_none() {
                reasons.push("continent");
            }
            if c.id.is_empty() {
                if c.eligible(&Settings::default()) {
                    return Err(format!("Eligible country missing ISO ID: {name}").into());
                }
                reasons.push("ISO ID");
            }
            if !reasons.is_empty() {
                excluded.insert(name.clone(), reasons);
            }
        }
        let eligible: Vec<_> = countries
            .iter()
            .filter(|c| c.eligible(&Settings::default()))
            .map(|c| c.id.clone())
            .collect();
        for c in countries
            .iter()
            .filter(|c| c.eligible(&Settings::default()))
        {
            if c.location.is_none() {
                return Err(format!("{}: eligible country has no map location", c.name).into());
            }
        }
        let marker_only: Vec<_> = eligible
            .iter()
            .filter(|id| !map.iter().any(|r| &r.id == *id))
            .cloned()
            .collect();
        Ok(Self {
            eligible,
            excluded,
            marker_only,
        })
    }
}
