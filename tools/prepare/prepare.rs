use crate::{Result, catalog, countries, coverage::Coverage, dataset, map, output::output};
use std::path::Path;

pub(crate) fn run(root: &Path, check: bool) -> Result<()> {
    let catalog = catalog::load(root)?;
    let countries = countries::load(root, &catalog.sources)?;
    let map = map::load(root, &catalog.map)?;
    let coverage = Coverage::build(&countries, &map)?;
    let data = dataset::build(&catalog.sources, countries)?;
    output(root, "assets/countries.json", &data, check)?;
    output(root, "assets/map.json", &map, check)?;
    output(root, "data/coverage.json", &coverage, check)?;
    println!(
        "{} countries, {} eligible, {} map regions. Assets {}.",
        data.countries.len(),
        coverage.eligible.len(),
        map.len(),
        if check { "verified" } else { "prepared" }
    );
    Ok(())
}
