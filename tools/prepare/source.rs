use super::{Result, input::Input, normalization, raw::checked_read};
use polars::prelude::*;
use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

pub(super) fn load(root: &Path, input: &Input) -> Result<DataFrame> {
    normalization::validate(input)?;
    let text = checked_read(root, &input.file, &input.sha256)?;
    let rows = read_rows(&text, input)?;
    let df = to_frame(&rows, input)?;
    let df = normalization::apply(df, input)?;
    validate_countries(&df, input)?;
    Ok(df)
}

fn read_rows(text: &str, input: &Input) -> Result<Vec<BTreeMap<String, String>>> {
    Ok(if input.normalization == "culture" {
        let raw: Vec<BTreeMap<String, Value>> = serde_json::from_str(text)?;
        raw.into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|(k, v)| {
                        let value = match v {
                            Value::Null => String::new(),
                            Value::String(s) => s,
                            other => other.to_string(),
                        };
                        (k, value)
                    })
                    .collect()
            })
            .collect()
    } else {
        let mut reader = csv::Reader::from_reader(text.trim_start_matches('\u{feff}').as_bytes());
        let headers = reader.headers()?.clone();
        reader
            .records()
            .map(|row| {
                Ok(headers
                    .iter()
                    .map(String::from)
                    .zip(row?.iter().map(String::from))
                    .collect())
            })
            .collect::<Result<_>>()?
    })
}

fn to_frame(rows: &[BTreeMap<String, String>], input: &Input) -> Result<DataFrame> {
    let mut columns = Vec::new();
    for (source, target) in &input.columns {
        let strings: Vec<Option<String>> = rows
            .iter()
            .map(|row| {
                let v = row
                    .get(source)
                    .ok_or_else(|| format!("{}: missing column {source}", input.file))?;
                Ok(if input.null_markers.contains(v) {
                    None
                } else {
                    Some(v.clone())
                })
            })
            .collect::<Result<_>>()?;
        if ["country", "id", "continent"].contains(&target.as_str()) {
            let strings: Vec<_> = strings
                .into_iter()
                .map(|v| {
                    v.map(|s| {
                        if target == "country" {
                            s.replace('*', "")
                        } else {
                            s
                        }
                    })
                })
                .collect();
            columns.push(Column::new(target.into(), strings));
        } else {
            let values: Vec<Option<f64>> = strings
                .into_iter()
                .enumerate()
                .map(|(i, value)| {
                    value
                        .map(|v| {
                            let number: f64 = v.parse().map_err(|_| {
                                format!("{} row {}: invalid {source}: {v}", input.file, i + 2)
                            })?;
                            if !number.is_finite() {
                                return Err(format!(
                                    "{} row {}: non-finite {source}",
                                    input.file,
                                    i + 2
                                )
                                .into());
                            }
                            Ok(number)
                        })
                        .transpose()
                })
                .collect::<Result<_>>()?;
            columns.push(Column::new(target.into(), values));
        }
    }
    Ok(DataFrame::new(rows.len(), columns)?)
}

fn validate_countries(df: &DataFrame, input: &Input) -> Result<()> {
    let countries = df.column("country")?.str()?;
    let mut unique = BTreeSet::new();
    for i in 0..countries.len() {
        let country = countries.get(i);
        let country = country.ok_or_else(|| format!("{}: missing country", input.id))?;
        if !unique.insert(country) {
            return Err(format!("{}: duplicate country {country}", input.id).into());
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "source_test.rs"]
mod tests;
