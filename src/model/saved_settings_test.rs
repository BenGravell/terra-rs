use crate::model::{Culture, Minimums, Overall, Quality, Range, SavedSettings, Settings};
use serde_json::json;

#[test]
fn corrupt_or_unsupported_saved_settings_are_rejected() {
    assert!(SavedSettings::decode("broken JSON").is_err());
    let mut saved = SavedSettings {
        version: 1,
        settings: Settings::default(),
    };
    assert_eq!(
        SavedSettings::decode(&serde_json::to_string(&saved).unwrap()).unwrap(),
        saved.settings
    );
    saved.version = 2;
    assert!(SavedSettings::decode(&serde_json::to_string(&saved).unwrap()).is_err());
}

#[test]
fn settings_round_trip_preserves_field_meanings_and_json_shape() {
    let original = json!({
        "version": 1,
        "settings": {
            "culture": [10., 20., 30., 40., 50., 60.],
            "overall_weights": [0.25, 0.75],
            "quality_weights": [0.2, 0.3, 0.5],
            "minimums": [0.1, 0.2, 0.3, 0.4, 0.5],
            "english_min": 0.6,
            "temperature": [-5., 25.],
            "sunshine": [4., 9.],
            "continents": ["Europe", "Asia"]
        }
    });
    let settings = SavedSettings::decode(&original.to_string()).unwrap();
    assert_eq!(
        settings.culture,
        Culture {
            power_distance: 10.,
            individualism: 20.,
            masculinity: 30.,
            uncertainty_avoidance: 40.,
            long_term_orientation: 50.,
            indulgence: 60.,
        }
    );
    assert_eq!(
        settings.overall_weights,
        Overall {
            culture: 0.25,
            quality: 0.75
        }
    );
    assert_eq!(
        settings.quality_weights,
        Quality {
            happy_planet: 0.2,
            social_progress: 0.3,
            human_freedom: 0.5,
        }
    );
    assert_eq!(
        settings.minimums,
        Minimums {
            culture: 0.1,
            quality: 0.2,
            happy_planet: 0.3,
            social_progress: 0.4,
            human_freedom: 0.5,
        }
    );
    assert_eq!(settings.temperature, Range { min: -5., max: 25. });
    assert_eq!(settings.sunshine, Range { min: 4., max: 9. });
    for (field, values) in [
        ("culture", settings.culture.values().to_vec()),
        (
            "overall_weights",
            settings.overall_weights.values().to_vec(),
        ),
        (
            "quality_weights",
            settings.quality_weights.values().to_vec(),
        ),
        ("minimums", settings.minimums.values().to_vec()),
        ("temperature", settings.temperature.values().to_vec()),
        ("sunshine", settings.sunshine.values().to_vec()),
    ] {
        assert_eq!(json!(values), original["settings"][field], "{field}");
    }
    assert_eq!(
        serde_json::to_value(SavedSettings {
            version: 1,
            settings
        })
        .unwrap(),
        original
    );

    for field in [
        "culture",
        "overall_weights",
        "quality_weights",
        "minimums",
        "temperature",
        "sunshine",
    ] {
        for extra in [false, true] {
            let mut invalid = original.clone();
            let values = invalid["settings"][field].as_array_mut().unwrap();
            if extra {
                values.push(json!(0.));
            } else {
                values.pop();
            }
            assert!(
                SavedSettings::decode(&invalid.to_string()).is_err(),
                "{field}, extra element: {extra}"
            );
        }
    }
}
