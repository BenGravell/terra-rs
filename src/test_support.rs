use crate::model::{Country, Dataset, Location, Quality, Settings};

pub(crate) fn dataset() -> Dataset {
    Dataset::parse(include_str!("../assets/countries.json")).unwrap()
}
pub(crate) fn sample() -> Country {
    Country {
        id: "AAA".into(),
        name: "Sample".into(),
        culture: Some(Settings::default().culture),
        quality: Some(Quality {
            happy_planet: 0.8,
            social_progress: 0.8,
            human_freedom: 0.8,
        }),
        english: Some(0.5),
        temperature: Some(20.),
        sunshine: Some(6.),
        continent: Some("Europe".into()),
        location: Some(Location {
            longitude: 0.,
            latitude: 0.,
        }),
    }
}
