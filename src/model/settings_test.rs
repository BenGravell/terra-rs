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
