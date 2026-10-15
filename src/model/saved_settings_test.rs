use crate::model::{Culture, Minimums, Overall, Quality, Range, SavedSettings, Settings};
use serde_json::json;

#[test]
fn corrupt_or_unsupported_saved_settings_are_rejected() {
    assert!(SavedSettings::decode("broken JSON").is_err());
    let mut saved = SavedSettings {
        version: 2,
        settings: Settings::default(),
    };
    assert_eq!(
        SavedSettings::decode(&serde_json::to_string(&saved).unwrap()).unwrap(),
        saved.settings
    );
    saved.version = 3;
    assert!(SavedSettings::decode(&serde_json::to_string(&saved).unwrap()).is_err());
}

#[test]
fn settings_round_trip_preserves_field_meanings_and_json_shape() {
    let original = json!({
        "version": 2,
        "settings": {
            "culture": [10., 20., 30., 40., 50., 60.],
            "overall_importances": [1., 3.],
            "quality_importances": [2., 3., 5.],
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
        settings.overall_importances,
        Overall {
            culture: 1.,
            quality: 3.
        }
    );
    assert_eq!(
        settings.quality_importances,
        Quality {
            happy_planet: 2.,
            social_progress: 3.,
            human_freedom: 5.,
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
            "overall_importances",
            settings.overall_importances.values().to_vec(),
        ),
        (
            "quality_importances",
            settings.quality_importances.values().to_vec(),
        ),
        ("minimums", settings.minimums.values().to_vec()),
        ("temperature", settings.temperature.values().to_vec()),
        ("sunshine", settings.sunshine.values().to_vec()),
    ] {
        assert_eq!(json!(values), original["settings"][field], "{field}");
    }
    assert_eq!(
        serde_json::to_value(SavedSettings {
            version: 2,
            settings
        })
        .unwrap(),
        original
    );

    for field in [
        "culture",
        "overall_importances",
        "quality_importances",
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

#[test]
fn legacy_weights_migrate_to_integer_importances() {
    let mut saved = serde_json::to_value(SavedSettings {
        version: 1,
        settings: Settings::default(),
    })
    .unwrap();
    let settings = saved["settings"].as_object_mut().unwrap();
    settings.remove("overall_importances");
    settings.remove("quality_importances");
    settings.insert("overall_weights".into(), json!([0., 1.]));
    settings.insert("quality_weights".into(), json!([0.3, 0.4, 0.3]));
    let migrated = SavedSettings::decode(&saved.to_string()).unwrap();
    assert_eq!(migrated.overall_importances.values(), [0., 10.]);
    assert_eq!(migrated.quality_importances.values(), [8., 10., 8.]);
    saved["settings"]["overall_weights"] = json!([0., 0.]);
    assert!(SavedSettings::decode(&saved.to_string()).is_err());
}
