use super::evaluate;
use crate::{
    model::{Culture, Overall, Quality, Range, Settings},
    test_support::{dataset, sample},
};

fn close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() < 1e-9, "{actual} != {expected}");
}

#[test]
fn scores_and_contributions_follow_the_weighted_formulas() {
    let mut country = sample();
    country.culture = Some(Culture {
        power_distance: 20.,
        individualism: 30.,
        masculinity: 40.,
        uncertainty_avoidance: 60.,
        long_term_orientation: 70.,
        indulgence: 80.,
    });
    country.quality = Some(Quality {
        happy_planet: 0.2,
        social_progress: 0.6,
        human_freedom: 0.9,
    });
    let settings = Settings {
        overall_weights: Overall {
            culture: 0.25,
            quality: 0.75,
        },
        quality_weights: Quality {
            happy_planet: 0.2,
            social_progress: 0.3,
            human_freedom: 0.5,
        },
        ..Settings::default()
    };
    let scores = evaluate(&[country], &settings).unwrap();
    assert_eq!(scores.len(), 1);
    let score = &scores[0];
    // Average distance from the default culture preference is 20 points.
    close(score.culture, 0.8);
    close(score.quality_contributions.happy_planet, 0.04);
    close(score.quality_contributions.social_progress, 0.18);
    close(score.quality_contributions.human_freedom, 0.45);
    close(score.quality, 0.67);
    close(score.overall_contributions.culture, 0.2);
    close(score.overall_contributions.quality, 0.5025);
    close(score.overall, 0.7025);
    assert_eq!(score.rank, 1);
    assert!(score.matches);
}

#[test]
fn ties_use_competition_ranks_and_filtering_preserves_rank() {
    let mut a = sample();
    a.id = "BBB".into();
    let b = sample();
    let mut c = sample();
    c.id = "CCC".into();
    c.culture = Some(Culture {
        power_distance: 0.,
        individualism: 0.,
        masculinity: 0.,
        uncertainty_avoidance: 0.,
        long_term_orientation: 0.,
        indulgence: 0.,
    });
    let countries = [a, b, c];
    let mut settings = Settings::default();
    let results = evaluate(&countries, &settings).unwrap();
    assert_eq!(
        results.iter().map(|s| s.rank).collect::<Vec<_>>(),
        [1, 1, 3]
    );
    assert_eq!(countries[results[0].country].id, "AAA");
    settings.minimums.culture = 0.5;
    let filtered = evaluate(&countries, &settings).unwrap();
    assert!(!filtered[2].matches);
    assert_eq!(filtered[2].rank, 3);
}

#[test]
fn missing_data_and_exact_boundaries_are_not_zero_filled_or_inclusive() {
    let mut country = sample();
    let mut settings = Settings {
        english_min: 0.5,
        ..Settings::default()
    };
    assert!(!evaluate(&[country.clone()], &settings).unwrap()[0].matches);
    country.english = None;
    assert!(evaluate(&[country.clone()], &settings).unwrap().is_empty());
    settings.english_min = 0.001;
    assert_eq!(evaluate(&[country.clone()], &settings).unwrap().len(), 1);
    settings.temperature = Range { min: 20., max: 30. };
    assert!(!evaluate(&[country.clone()], &settings).unwrap()[0].matches);
    settings = Settings::default();
    country.temperature = None;
    assert!(evaluate(&[country], &settings).unwrap().is_empty());
}

#[test]
fn scoring_full_dataset_stays_within_interaction_budget() {
    let data = dataset();
    let start = std::time::Instant::now();
    for _ in 0..100 {
        std::hint::black_box(evaluate(&data.countries, &Settings::default()).unwrap());
    }
    let average = start.elapsed() / 100;
    eprintln!("Average full-dataset recomputation: {average:?}");
    assert!(average.as_millis() < 100);
}
