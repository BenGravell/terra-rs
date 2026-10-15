use crate::model::{Country, Culture, EPS, Overall, Quality, Range, Score, Settings};

pub fn evaluate(countries: &[Country], settings: &Settings) -> Result<Vec<Score>, String> {
    settings.validate()?;
    let mut results = score_eligible_countries(countries, settings);
    sort_by_overall_score(&mut results, countries);
    assign_matching_ranks(&mut results);
    Ok(results)
}

fn score_eligible_countries(countries: &[Country], settings: &Settings) -> Vec<Score> {
    countries
        .iter()
        .enumerate()
        .filter(|(_, country)| country.eligible(settings))
        .map(|(index, country)| score_country(index, country, settings))
        .collect()
}

// Eligibility guarantees the country data required for scoring and constraints is present.
fn score_country(index: usize, country: &Country, settings: &Settings) -> Score {
    let culture = culture_fit(country.culture.unwrap(), settings.culture);
    let quality_contributions =
        quality_contributions(country.quality.unwrap(), settings.quality_importances);
    let quality = quality_contributions.values().iter().sum();
    let components = Overall { culture, quality };
    let overall_contributions = overall_contributions(components, settings.overall_importances);
    Score {
        country: index,
        rank: 0,
        culture,
        quality,
        overall: overall_contributions.values().iter().sum(),
        quality_contributions,
        overall_contributions,
        matches: matches_constraints(country, components, settings),
    }
}

fn culture_fit(country: Culture, preferences: Culture) -> f64 {
    let preferences = preferences.values();
    1. - country
        .values()
        .into_iter()
        .zip(preferences)
        .map(|(value, preference)| (value - preference).abs())
        .sum::<f64>()
        / 100.
        / preferences.len() as f64
}

fn quality_contributions(quality: Quality, weights: Quality) -> Quality {
    let total: f64 = weights.values().iter().sum();
    Quality {
        happy_planet: quality.happy_planet * (weights.happy_planet / total),
        social_progress: quality.social_progress * (weights.social_progress / total),
        human_freedom: quality.human_freedom * (weights.human_freedom / total),
    }
}

fn overall_contributions(scores: Overall, weights: Overall) -> Overall {
    let total: f64 = weights.values().iter().sum();
    Overall {
        culture: scores.culture * (weights.culture / total),
        quality: scores.quality * (weights.quality / total),
    }
}

fn matches_constraints(country: &Country, scores: Overall, settings: &Settings) -> bool {
    meets_minimums(country, scores, settings)
        && matches_climate_range(
            country.temperature.unwrap(),
            settings.temperature,
            Range {
                min: -10.,
                max: 30.,
            },
        )
        && matches_climate_range(
            country.sunshine.unwrap(),
            settings.sunshine,
            Range { min: 3., max: 10. },
        )
        && settings
            .continents
            .contains(country.continent.as_ref().unwrap())
}

fn meets_minimums(country: &Country, scores: Overall, settings: &Settings) -> bool {
    let quality = country.quality.unwrap();
    let minimums = settings.minimums;
    [
        (Some(scores.culture), minimums.culture),
        (Some(scores.quality), minimums.quality),
        (Some(quality.happy_planet), minimums.happy_planet),
        (Some(quality.social_progress), minimums.social_progress),
        (Some(quality.human_freedom), minimums.human_freedom),
        (country.english, settings.english_min),
    ]
    .into_iter()
    .all(|(value, minimum)| minimum <= EPS || value.is_some_and(|value| value > minimum))
}

fn matches_climate_range(value: f64, range: Range, default: Range) -> bool {
    let active = range.min > default.min + EPS || range.max < default.max - EPS;
    !active || (value > range.min && value < range.max)
}

fn sort_by_overall_score(results: &mut [Score], countries: &[Country]) {
    results.sort_by(|a, b| {
        b.overall
            .total_cmp(&a.overall)
            .then_with(|| countries[a.country].id.cmp(&countries[b.country].id))
    });
}

fn assign_matching_ranks(results: &mut [Score]) {
    for (index, score) in results.iter_mut().filter(|score| score.matches).enumerate() {
        score.rank = index + 1;
    }
}

#[cfg(test)]
#[path = "scoring_test.rs"]
mod tests;
