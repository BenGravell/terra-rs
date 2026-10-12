use super::Result;
use polars::prelude::*;
use terra::model::{Country, Culture, Location, Quality};

pub(super) fn from_row(joined: &DataFrame, i: usize) -> Result<Country> {
    let string = |field: &str| -> Result<Option<String>> {
        Ok(joined.column(field)?.str()?.get(i).map(str::to_string))
    };
    let number = |field: &str| -> Result<Option<f64>> { Ok(joined.column(field)?.f64()?.get(i)) };
    let name = string("country")?.ok_or("Missing joined country")?;
    let culture_value = |field| -> Result<Option<f64>> { Ok(number(field)?.filter(|x| *x >= 0.)) };
    let culture = match (
        culture_value("pdi")?,
        culture_value("idv")?,
        culture_value("mas")?,
        culture_value("uai")?,
        culture_value("lto")?,
        culture_value("ind")?,
    ) {
        (
            Some(power_distance),
            Some(individualism),
            Some(masculinity),
            Some(uncertainty_avoidance),
            Some(long_term_orientation),
            Some(indulgence),
        ) => Some(Culture {
            power_distance,
            individualism,
            masculinity,
            uncertainty_avoidance,
            long_term_orientation,
            indulgence,
        }),
        _ => None,
    };
    let quality = match (number("hp")?, number("sp")?, number("hf")?) {
        (Some(happy_planet), Some(social_progress), Some(human_freedom)) => Some(Quality {
            happy_planet,
            social_progress,
            human_freedom,
        }),
        _ => None,
    };
    Ok(Country {
        id: string("id")?.unwrap_or_default(),
        name,
        culture,
        quality,
        english: number("english")?,
        temperature: number("temperature")?,
        sunshine: number("sunshine")?,
        continent: string("continent")?,
        location: number("longitude")?
            .zip(number("latitude")?)
            .map(|(longitude, latitude)| Location {
                longitude: longitude as f32,
                latitude: latitude as f32,
            }),
    })
}
