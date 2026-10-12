use super::theme::{ACCENT, CULTURE, PARTS, QUALITY, QUALITY_NAMES, percent};
use crate::model::{Country, Score};
use bevy_egui::egui::{self, Color32, RichText, Stroke, Vec2 as EVec2};

pub(super) fn show(
    ui: &mut egui::Ui,
    countries: &[Country],
    scores: &[Score],
    selected: &mut Option<usize>,
    bar_count: &mut usize,
) {
    ui.heading("How the scores add up");
    ui.add(egui::Slider::new(bar_count, 1..=30).text("Countries"));
    ui.horizontal_wrapped(|ui| {
        ui.colored_label(CULTURE, "■ Culture Fit");
        ui.colored_label(QUALITY, "■ Quality of Life");
    });
    let mut indices: Vec<_> = scores
        .iter()
        .enumerate()
        .filter(|(_, s)| s.matches)
        .take(*bar_count)
        .map(|(i, _)| i)
        .collect();
    if let Some(i) = scores.iter().position(|s| Some(s.country) == *selected)
        && !indices.contains(&i)
    {
        indices.push(i);
    }
    for i in indices {
        let s = &scores[i];
        let name = format!("{}. {}", s.rank, countries[s.country].name);
        if stacked_bar(
            ui,
            &name,
            &s.overall_contributions.values(),
            &[CULTURE, QUALITY],
            *selected == Some(s.country),
            &["Culture Fit", "Quality of Life"],
        ) {
            *selected = Some(s.country);
        }
    }
    if let Some(s) = scores.iter().find(|s| Some(s.country) == *selected) {
        ui.add_space(8.);
        ui.strong(format!("{} · Quality of Life", countries[s.country].name));
        ui.horizontal_wrapped(|ui| {
            for (name, color) in QUALITY_NAMES.iter().zip(PARTS) {
                ui.colored_label(color, *name);
            }
        });
        stacked_bar(
            ui,
            "Quality of Life",
            &s.quality_contributions.values(),
            &PARTS,
            false,
            &QUALITY_NAMES,
        );
        ui.small(format!(
            "{} of Quality of Life contributes {} to the overall score.",
            percent(s.quality),
            percent(s.overall_contributions.quality)
        ));
    }
}

fn stacked_bar(
    ui: &mut egui::Ui,
    label: &str,
    values: &[f64],
    colors: &[Color32],
    selected: bool,
    names: &[&str],
) -> bool {
    let total: f64 = values.iter().sum();
    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new(label).color(if selected {
            ACCENT
        } else {
            ui.visuals().text_color()
        }));
        ui.label(percent(total));
    });
    let (rect, response) =
        ui.allocate_exact_size(EVec2::new(ui.available_width(), 16.), egui::Sense::click());
    ui.painter()
        .rect_filled(rect, 2., Color32::from_rgb(36, 49, 62));
    let mut x = rect.left();
    for (value, color) in values.iter().zip(colors) {
        let w = rect.width() * *value as f32;
        ui.painter().rect_filled(
            egui::Rect::from_min_size(egui::pos2(x, rect.top()), EVec2::new(w, rect.height())),
            1.,
            *color,
        );
        x += w;
    }
    if selected {
        ui.painter().rect_stroke(
            rect,
            2.,
            Stroke::new(1., Color32::WHITE),
            egui::StrokeKind::Outside,
        );
    }
    let clicked = response.clicked();
    response.on_hover_text(
        names
            .iter()
            .zip(values)
            .map(|(name, value)| format!("{name}: {}", percent(*value)))
            .collect::<Vec<_>>()
            .join("\n"),
    );
    clicked
}
