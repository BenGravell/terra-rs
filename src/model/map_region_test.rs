use crate::model::{MapPoint, MapRegion};

fn regions() -> Vec<MapRegion> {
    MapRegion::parse(include_str!("../../assets/map.json")).unwrap()
}
fn projected(lon: f32, lat: f32) -> MapPoint {
    MapPoint {
        x: (lon + 180.) / 360.,
        y: (90. - lat) / 180.,
    }
}

#[test]
fn holes_and_both_sides_of_the_antimeridian_are_preserved() {
    let regions = regions();
    assert_eq!(regions.len(), 177);
    let south_africa = regions.iter().find(|r| r.id == "ZAF").unwrap();
    assert!(south_africa.contains(projected(25., -30.)));
    assert!(
        !south_africa.contains(projected(28.5, -29.5)),
        "Lesotho must remain a hole in South Africa"
    );
    let fiji = regions.iter().find(|r| r.id == "FJI").unwrap();
    assert!(fiji.contains(projected(178., -17.8)));
    assert!(fiji.contains(projected(-179.9, -16.3)));
    assert!(
        !fiji.contains(projected(0., -17.)),
        "Fiji must not stretch across the globe"
    );
}

#[test]
fn corrupt_indices_and_empty_maps_fail_before_rendering() {
    assert!(MapRegion::parse("[]").is_err());
    assert!(
        MapRegion::parse(r#"[{"id":"X","name":"Broken","points":[[0,0]],"indices":[0,1,2]}]"#)
            .is_err()
    );
}
