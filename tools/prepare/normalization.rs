use super::{Result, input::Input};
use polars::prelude::*;

pub(super) fn validate(input: &Input) -> Result<()> {
    if !["none", "culture", "max", "max_then_year_mean"].contains(&input.normalization.as_str()) {
        return Err(format!(
            "{}: unknown normalization {}",
            input.id, input.normalization
        )
        .into());
    }
    Ok(())
}

pub(super) fn apply(mut df: DataFrame, input: &Input) -> Result<DataFrame> {
    if input.normalization == "max" || input.normalization == "max_then_year_mean" {
        let field = input
            .columns
            .values()
            .find(|s| !["country", "year"].contains(&s.as_str()))
            .ok_or("Missing score column")?;
        let maximum = df
            .column(field)?
            .f64()?
            .max()
            .ok_or("No scores to normalize")?;
        if maximum <= 0. {
            return Err(format!("{}: normalization maximum must be positive", input.id).into());
        }
        df = df
            .lazy()
            .with_column((col(field.as_str()) / lit(maximum)).alias(field.as_str()))
            .collect()?;
        if input.normalization == "max_then_year_mean" {
            df = df
                .lazy()
                .filter(
                    col("year")
                        .gt_eq(lit(input.year_min.ok_or("Missing year_min")?))
                        .and(col("year").lt_eq(lit(input.year_max.ok_or("Missing year_max")?))),
                )
                .group_by([col("country")])
                .agg([col(field.as_str()).mean()])
                .collect()?;
        }
    }
    Ok(df)
}
