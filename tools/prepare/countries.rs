use super::{Result, country, input::Input, source};
use polars::prelude::*;
use std::{collections::BTreeSet, path::Path};
use terra::model::Country;

pub(super) fn load(root: &Path, inputs: &[Input]) -> Result<Vec<Country>> {
    let frames: Vec<_> = inputs
        .iter()
        .map(|s| source::load(root, s))
        .collect::<Result<_>>()?;
    let names: BTreeSet<_> = frames
        .iter()
        .flat_map(|f| {
            (0..f.height()).map(|i| f.column("country").unwrap().str().unwrap().get(i).unwrap())
        })
        .collect();
    let mut joined = DataFrame::new(
        names.len(),
        vec![Column::new(
            "country".into(),
            names.into_iter().collect::<Vec<_>>(),
        )],
    )?
    .lazy();
    for frame in frames {
        joined = joined.join(
            frame.lazy(),
            [col("country")],
            [col("country")],
            JoinArgs::new(JoinType::Left),
        );
    }
    let joined = joined.collect()?;
    let mut countries = (0..joined.height())
        .map(|i| country::from_row(&joined, i))
        .collect::<Result<Vec<_>>>()?;
    countries.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(countries)
}
