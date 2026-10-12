use crate::model::{Overall, Quality, Settings, adjust_weight};

fn close(actual: &[f64], expected: &[f64]) {
    assert_eq!(actual.len(), expected.len());
    for (a, b) in actual.iter().zip(expected) {
        assert!((a - b).abs() < 1e-12, "{actual:?} != {expected:?}");
    }
}

#[test]
fn shares_changes_equally_and_redistributes_past_empty_peers() {
    let mut weights = Settings::default().quality_weights;
    for (value, expected) in [
        (0.35, [0.35, 0.375, 0.275]),
        (0.3, [0.3, 0.4, 0.3]),
        (0.9, [0.9, 0.1, 0.]),
        (1., [1., 0., 0.]),
        (0., [0., 0.5, 0.5]),
    ] {
        adjust_weight(
            &mut weights.happy_planet,
            &mut [&mut weights.social_progress, &mut weights.human_freedom],
            value,
        )
        .unwrap();
        close(&weights.values(), &expected);
    }

    let mut overall = Settings::default().overall_weights;
    for value in [0., 0.35, 1., 0.5] {
        adjust_weight(&mut overall.quality, &mut [&mut overall.culture], value).unwrap();
        close(&overall.values(), &[1. - value, value]);
    }
}

#[test]
fn repeated_edits_preserve_bounds_selected_value_and_actual_total() {
    let mut settings = Settings::default();
    for step in 0..3000 {
        let weights = &mut settings.quality_weights;
        let (selected, mut peers) = match step % 3 {
            0 => (
                &mut weights.happy_planet,
                [&mut weights.social_progress, &mut weights.human_freedom],
            ),
            1 => (
                &mut weights.social_progress,
                [&mut weights.happy_planet, &mut weights.human_freedom],
            ),
            _ => (
                &mut weights.human_freedom,
                [&mut weights.happy_planet, &mut weights.social_progress],
            ),
        };
        let value = ((step * 37) % 101) as f64 / 100.;
        adjust_weight(selected, &mut peers, value).unwrap();
        assert_eq!(*selected, value);
        let values = settings.quality_weights.values();
        assert!(values.iter().all(|w| (0.0..=1.0).contains(w)));
        close(&[values.iter().sum()], &[1.]);
        settings.validate().unwrap();
    }
    // Equal redistribution can produce percentages that round to 101 or 99 in total.
    for weights in [
        Quality {
            happy_planet: 0.35,
            social_progress: 0.375,
            human_freedom: 0.275,
        },
        Quality {
            happy_planet: 0.3333,
            social_progress: 0.3333,
            human_freedom: 0.3334,
        },
    ] {
        settings.quality_weights = weights;
        settings.validate().unwrap();
    }
    settings.overall_weights = Overall {
        culture: 0.501,
        quality: 0.501,
    };
    assert!(settings.validate().is_err());
}

#[test]
fn invalid_adjustments_leave_weights_unchanged() {
    for value in [-0.1, 1.1, f64::NAN, f64::INFINITY] {
        let mut weights = Settings::default().overall_weights;
        assert!(adjust_weight(&mut weights.culture, &mut [&mut weights.quality], value).is_err());
        assert_eq!(weights, Settings::default().overall_weights);
    }
    let mut selected = 1.;
    assert!(adjust_weight(&mut selected, &mut [], 0.5).is_err());
    assert_eq!(selected, 1.);
    let mut invalid = f64::NAN;
    assert!(adjust_weight(&mut selected, &mut [&mut invalid], 0.5).is_err());
    assert_eq!(selected, 1.);
    assert!(invalid.is_nan());
    assert!(adjust_weight(&mut invalid, &mut [&mut selected], 0.5).is_err());
    assert!(invalid.is_nan());
    assert_eq!(selected, 1.);
}
