use super::{Result, input::Input};
use terra::model::{Country, Dataset, Source};

pub(super) fn build(inputs: &[Input], countries: Vec<Country>) -> Result<Dataset> {
    let data = Dataset {
        schema_version: 1,
        sources: inputs
            .iter()
            .map(|s| Source {
                name: s.name.clone(),
                url: s.url.clone(),
                year: s.year.clone(),
            })
            .collect(),
        countries: countries.into_iter().filter(|c| !c.id.is_empty()).collect(),
    };
    Dataset::parse(&serde_json::to_string(&data)?)?;
    Ok(data)
}
