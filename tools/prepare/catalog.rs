use super::{Result, input::Input, map_input::MapInput};
use serde::Deserialize;
use std::{collections::BTreeSet, fs, path::Path};

#[derive(Deserialize)]
pub(super) struct Catalog {
    pub sources: Vec<Input>,
    pub map: MapInput,
}

pub(super) fn load(root: &Path) -> Result<Catalog> {
    let catalog: Catalog = toml::from_str(&fs::read_to_string(root.join("data/catalog.toml"))?)?;
    let mut fields = BTreeSet::new();
    for source in &catalog.sources {
        for field in source
            .columns
            .values()
            .filter(|field| field.as_str() != "country")
        {
            if !fields.insert(field) {
                return Err(format!("Duplicate catalog output column {field}; update the existing source rather than shadowing it").into());
            }
        }
    }
    if !catalog.sources.iter().any(|s| s.id == "codes") {
        return Err("Missing country code source".into());
    }
    Ok(catalog)
}
