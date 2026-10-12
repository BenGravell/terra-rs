use crate::{
    model::{Dataset, Settings},
    test_support::dataset,
};

#[test]
fn dataset_rejects_duplicate_ids_and_invalid_values() {
    let mut data = dataset();
    data.countries.push(data.countries[0].clone());
    assert!(Dataset::parse(&serde_json::to_string(&data).unwrap()).is_err());
    data.countries.pop();
    data.countries[0].english = Some(2.);
    assert!(Dataset::parse(&serde_json::to_string(&data).unwrap()).is_err());
}

#[test]
fn every_eligible_country_can_be_located() {
    let countries = dataset().countries;
    for c in countries
        .iter()
        .filter(|c| c.eligible(&Settings::default()))
    {
        assert!(c.location.is_some(), "{} needs a marker", c.id);
    }
}
