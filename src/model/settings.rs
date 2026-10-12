use super::{CONTINENTS, Country, Culture, Minimums, Overall, Quality, Range};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Settings {
    pub culture: Culture,
    pub overall_weights: Overall,
    pub quality_weights: Quality,
    pub minimums: Minimums,
    pub english_min: f64,
    pub temperature: Range,
    pub sunshine: Range,
    pub continents: Vec<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            culture: Culture {
                power_distance: 50.,
                individualism: 50.,
                masculinity: 50.,
                uncertainty_avoidance: 50.,
                long_term_orientation: 50.,
                indulgence: 50.,
            },
            overall_weights: Overall {
                culture: 0.5,
                quality: 0.5,
            },
            quality_weights: Quality {
                happy_planet: 0.3,
                social_progress: 0.4,
                human_freedom: 0.3,
            },
            minimums: Minimums {
                culture: 0.,
                quality: 0.,
                happy_planet: 0.,
                social_progress: 0.,
                human_freedom: 0.,
            },
            english_min: 0.,
            temperature: Range {
                min: -10.,
                max: 30.,
            },
            sunshine: Range { min: 3., max: 10. },
            continents: CONTINENTS.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl Settings {
    pub fn validate(&self) -> Result<(), String> {
        let bounded = |values: &[f64], low, high| {
            values
                .iter()
                .all(|x| x.is_finite() && *x >= low && *x <= high)
        };
        if !bounded(&self.culture.values(), 0., 100.) {
            return Err("Culture preferences must be between 0 and 100.".into());
        }
        for (name, weights) in [
            ("Overall", &self.overall_weights.values()[..]),
            ("Quality of Life", &self.quality_weights.values()[..]),
        ] {
            if !bounded(weights, 0., 1.) || (weights.iter().sum::<f64>() - 1.).abs() > 1e-9 {
                return Err(format!("{name} weights must add up to 100%."));
            }
        }
        if !bounded(&self.minimums.values(), 0., 1.) || !bounded(&[self.english_min], 0., 1.) {
            return Err("Minimum scores must be between 0% and 100%.".into());
        }
        if !bounded(&self.temperature.values(), -10., 30.)
            || !bounded(&self.sunshine.values(), 3., 10.)
            || self.temperature.min > self.temperature.max
            || self.sunshine.min > self.sunshine.max
        {
            return Err("Check the climate ranges: minimum cannot exceed maximum.".into());
        }
        if self
            .continents
            .iter()
            .any(|c| !CONTINENTS.contains(&c.as_str()))
            || self.continents.iter().collect::<HashSet<_>>().len() != self.continents.len()
        {
            return Err("Unknown or duplicate continent in settings.".into());
        }
        Ok(())
    }

    pub fn use_culture(&mut self, country: &Country) {
        if let Some(culture) = country.culture {
            self.culture = culture;
        }
    }

    pub fn use_quality_thresholds(&mut self, country: &Country) {
        if let Some(quality) = country.quality {
            let floor_percent = |score: f64| (score * 100.).floor() / 100.;
            self.minimums.happy_planet = floor_percent(quality.happy_planet);
            self.minimums.social_progress = floor_percent(quality.social_progress);
            self.minimums.human_freedom = floor_percent(quality.human_freedom);
        }
    }
}

#[cfg(test)]
#[path = "settings_test.rs"]
mod tests;
