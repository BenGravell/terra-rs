use super::Settings;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SavedSettings {
    pub version: u32,
    pub settings: Settings,
}

impl SavedSettings {
    pub fn decode(text: &str) -> Result<Settings, String> {
        let saved: Self =
            serde_json::from_str(text).map_err(|_| "Saved settings could not be read.")?;
        if saved.version != 1 {
            return Err("Saved settings use an unsupported version.".into());
        }
        saved.settings.validate()?;
        Ok(saved.settings)
    }
}

#[cfg(test)]
#[path = "saved_settings_test.rs"]
mod tests;
