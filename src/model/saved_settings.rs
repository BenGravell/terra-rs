use super::Settings;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SavedSettings {
    pub version: u32,
    pub settings: Settings,
}

impl SavedSettings {
    pub fn decode(text: &str) -> Result<Settings, String> {
        let mut saved: Self =
            serde_json::from_str(text).map_err(|_| "Saved settings could not be read.")?;
        if saved.version == 1 {
            migrate_importances(&mut [
                &mut saved.settings.overall_importances.culture,
                &mut saved.settings.overall_importances.quality,
            ])?;
            migrate_importances(&mut [
                &mut saved.settings.quality_importances.happy_planet,
                &mut saved.settings.quality_importances.social_progress,
                &mut saved.settings.quality_importances.human_freedom,
            ])?;
        } else if saved.version != 2 {
            return Err("Saved settings use an unsupported version.".into());
        }
        saved.settings.validate()?;
        Ok(saved.settings)
    }
}

#[cfg(test)]
#[path = "saved_settings_test.rs"]
mod tests;

fn migrate_importances(values: &mut [&mut f64]) -> Result<(), String> {
    if values
        .iter()
        .any(|v| !v.is_finite() || !(0.0..=1.0).contains(*v))
        || (values.iter().map(|v| **v).sum::<f64>() - 1.).abs() > 1e-9
    {
        return Err("Invalid saved weights.".into());
    }
    let max = values.iter().map(|v| **v).fold(0., f64::max);
    for value in values {
        **value = (**value * 10. / max).round();
    }
    Ok(())
}
