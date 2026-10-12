use super::*;
use sha2::{Digest, Sha256};
use std::fs;

#[test]
fn rejects_corrupt_sources_and_normalizes_before_grouping() {
    let root = std::env::temp_dir().join(format!("terra-prepare-test-{}", std::process::id()));
    fs::create_dir_all(root.join("data/raw")).unwrap();
    let mut input = Input {
        id: "test".into(),
        file: "sample.csv".into(),
        sha256: String::new(),
        name: String::new(),
        url: String::new(),
        year: String::new(),
        normalization: "max_then_year_mean".into(),
        columns: BTreeMap::from([
            ("country".into(), "country".into()),
            ("score".into(), "hf".into()),
            ("year".into(), "year".into()),
        ]),
        null_markers: vec![String::new()],
        year_min: Some(2015.),
        year_max: Some(2020.),
    };
    let text = "country,score,year\nA,10,2014\nA,4,2015\nA,8,2016\nB,,2015\n";
    fs::write(root.join("data/raw/sample.csv"), text).unwrap();
    assert!(
        load(&root, &input)
            .unwrap_err()
            .to_string()
            .contains("checksum")
    );
    input.sha256 = format!("{:x}", Sha256::digest(text));
    let frame = load(&root, &input).unwrap();
    let row = (0..frame.height())
        .find(|i| frame.column("country").unwrap().str().unwrap().get(*i) == Some("A"))
        .unwrap();
    assert!((frame.column("hf").unwrap().f64().unwrap().get(row).unwrap() - 0.6).abs() < 1e-12);
    input.normalization = "none".into();
    assert!(
        load(&root, &input)
            .unwrap_err()
            .to_string()
            .contains("duplicate")
    );
    input.columns.insert("absent".into(), "absent".into());
    assert!(
        load(&root, &input)
            .unwrap_err()
            .to_string()
            .contains("missing column")
    );
    fs::remove_dir_all(root).unwrap();
}
