#[cfg(target_family = "wasm")]
use crate::model::SavedSettings;
use crate::model::Settings;

#[cfg(target_family = "wasm")]
pub(super) fn restore() -> (Settings, Option<String>) {
    let loaded = (|| {
        let storage = web_sys::window()
            .ok_or("Browser window unavailable")?
            .local_storage()
            .map_err(|_| "Local storage is unavailable; settings last for this visit.")?
            .ok_or("Local storage is unavailable; settings last for this visit.")?;
        let text = storage
            .get_item("terra.settings.v1")
            .map_err(|_| "Could not read saved settings.")?;
        match text {
            Some(text) => SavedSettings::decode(&text),
            None => Ok(Settings::default()),
        }
    })();
    match loaded {
        Ok(settings) => (settings, None),
        Err(error) => (
            Settings::default(),
            Some(format!("{error} Using defaults.")),
        ),
    }
}
#[cfg(not(target_family = "wasm"))]
pub(super) fn restore() -> (Settings, Option<String>) {
    (Settings::default(), None)
}
#[cfg(target_family = "wasm")]
pub(super) fn persist(settings: &Settings) -> Result<(), String> {
    let saved = serde_json::to_string(&SavedSettings {
        version: 2,
        settings: settings.clone(),
    })
    .map_err(|e| e.to_string())?;
    web_sys::window()
        .and_then(|w| w.local_storage().ok().flatten())
        .ok_or("Local storage is unavailable; settings last for this visit.")?
        .set_item("terra.settings.v1", &saved)
        .map_err(|_| "Could not save settings; changes last for this visit.".into())
}
#[cfg(not(target_family = "wasm"))]
pub(super) fn persist(_: &Settings) -> Result<(), String> {
    Ok(())
}
pub(super) fn ready() {
    #[cfg(target_family = "wasm")]
    if let Some(doc) = web_sys::window().and_then(|w| w.document())
        && let Some(root) = doc.document_element()
        && !root.has_attribute("data-terra-ready")
    {
        let _ = root.set_attribute("data-terra-ready", "true");
    }
}
