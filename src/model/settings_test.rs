use serde_json::json;

use crate::{
    model::{Culture, Minimums, Overall, Quality, Range, Settings},
    scoring::evaluate,
    test_support::sample,
};

#[test]
fn invalid_weights_and_climate_ranges_are_rejected() {
    for (culture, quality) in [(0., 0.), (0.4, 0.4), (f64::NAN, 1.), (-0.5, 1.5)] {
        let settings = Settings {
            overall_importances: Overall { culture, quality },
            ..Settings::default()
        };
        assert!(evaluate(&[sample()], &settings).is_err());
    }
    assert!(
        Settings {
            temperature: Range { min: 20., max: 10. },
            ..Settings::default()
        }
        .validate()
        .is_err()
    );
}

#[test]
fn reference_shortcuts_and_empty_continents() {
    let mut c = sample();
    c.quality = Some(Quality {
        happy_planet: 0.789,
        social_progress: 0.901,
        human_freedom: 0.555,
    });
    c.culture = Some(Culture {
        power_distance: 35.,
        individualism: 35.,
        masculinity: 35.,
        uncertainty_avoidance: 35.,
        long_term_orientation: 35.,
        indulgence: 35.,
    });
    let mut settings = Settings::default();
    settings.use_culture(&c);
    settings.use_quality_thresholds(&c);
    assert_eq!(Some(settings.culture), c.culture);
    assert_eq!(
        settings.minimums,
        Minimums {
            culture: 0.,
            quality: 0.,
            happy_planet: 0.78,
            social_progress: 0.90,
            human_freedom: 0.55
        }
    );
    settings.continents.clear();
    assert!(!evaluate(&[c], &settings).unwrap()[0].matches);
}

#[test]
fn importance_groups_reject_invalid_values_and_accept_single_nonzero_values() {
    for value in [-1., 0.5, 11., f64::NAN, f64::INFINITY] {
        let mut settings = Settings::default();
        settings.quality_importances.happy_planet = value;
        assert!(evaluate(&[sample()], &settings).is_err());
    }
    let mut settings = Settings {
        quality_importances: Quality {
            happy_planet: 0.,
            social_progress: 0.,
            human_freedom: 0.,
        },
        ..Settings::default()
    };
    assert!(evaluate(&[sample()], &settings).is_err());
    settings.quality_importances.happy_planet = 1.;
    settings.overall_importances = Overall {
        culture: 0.,
        quality: 10.,
    };
    let country = sample();
    let expected = country.quality.unwrap().happy_planet;
    let scores = evaluate(&[country], &settings).unwrap();
    assert_eq!(scores[0].overall, expected);
}

#[test]
fn settings_round_trip_preserves_field_meanings_and_json_shape() {
    let original = json!({
        "culture": [10., 20., 30., 40., 50., 60.],
        "overall_importances": [1., 3.],
        "quality_importances": [2., 3., 5.],
        "minimums": [0.1, 0.2, 0.3, 0.4, 0.5],
        "english_min": 0.6,
        "temperature": [-5., 25.],
        "sunshine": [4., 9.],
        "continents": ["Europe", "Asia"]
    });
    let settings = Settings::decode(&original.to_string()).unwrap();
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
        assert_eq!(json!(values), original[field], "{field}");
    }
    assert_eq!(serde_json::to_value(settings).unwrap(), original);

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
            let values = invalid[field].as_array_mut().unwrap();
            if extra {
                values.push(json!(0.));
            } else {
                values.pop();
            }
            assert!(
                Settings::decode(&invalid.to_string()).is_err(),
                "{field}, extra element: {extra}"
            );
        }
    }
}

#[test]
fn saved_settings_require_current_fields_and_valid_values() {
    assert!(Settings::decode("broken JSON").is_err());
    let original = serde_json::to_value(Settings::default()).unwrap();
    for (current, old) in [
        ("overall_importances", "overall_weights"),
        ("quality_importances", "quality_weights"),
    ] {
        let mut invalid = original.clone();
        let fields = invalid.as_object_mut().unwrap();
        let value = fields.remove(current).unwrap();
        fields.insert(old.into(), value);
        assert!(Settings::decode(&invalid.to_string()).is_err());
    }
    let mut invalid = original;
    invalid["overall_importances"] = json!([0.5, 1.]);
    assert!(Settings::decode(&invalid.to_string()).is_err());
}
