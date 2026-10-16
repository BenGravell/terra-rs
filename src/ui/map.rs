use super::theme::{ACCENT, percent, score_color};
use crate::model::{Country, MapPoint, MapRegion, Score};
use bevy_egui::egui::{self, Color32, RichText, Stroke, Vec2 as EVec2};

const INFEASIBLE: Color32 = Color32::from_rgb(188, 163, 150);
const NO_ELIGIBLE_DATA: Color32 = Color32::from_rgb(211, 220, 232);

pub(super) fn show(
    ui: &mut egui::Ui,
    countries: &[Country],
    regions: &[MapRegion],
    scores: &[Score],
    selected: &mut Option<usize>,
) {
    ui.add_space(8.);
    let width = ui.available_width();
    let map_area_width = (width - 92.).max(1.);
    let height = (map_area_width / 2.).max(100.);
    let range = visible_range(
        scores
            .iter()
            .filter(|s| {
                s.matches
                    && (countries[s.country].location.is_some()
                        || regions.iter().any(|r| r.id == countries[s.country].id))
            })
            .map(|s| s.overall),
    );
    let map_color = |value| score_color(normalize(value, range));
    let (rect, response) = ui.allocate_exact_size(EVec2::new(width, height), egui::Sense::click());
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 6., Color32::from_rgb(237, 244, 252));
    let map_width = (height * 2.).min(map_area_width);
    let view = egui::Rect::from_center_size(
        egui::pos2(rect.left() + map_area_width / 2., rect.center().y),
        EVec2::new(map_width, height),
    );
    let project = |p: MapPoint| {
        egui::pos2(
            view.left() + p.x * view.width(),
            view.top() + p.y * view.height(),
        )
    };
    for x in 1..6 {
        let x = view.left() + view.width() * x as f32 / 6.;
        painter.line_segment(
            [egui::pos2(x, view.top()), egui::pos2(x, view.bottom())],
            Stroke::new(0.5, Color32::from_rgb(212, 224, 239)),
        );
    }
    for y in 1..4 {
        let y = view.top() + view.height() * y as f32 / 4.;
        painter.line_segment(
            [egui::pos2(view.left(), y), egui::pos2(view.right(), y)],
            Stroke::new(0.5, Color32::from_rgb(212, 224, 239)),
        );
    }
    let pointer = response.hover_pos();
    let mut hovered: Option<(&str, Option<usize>)> = None;
    for region in regions {
        let country = countries.iter().position(|c| c.id == region.id);
        let score = country.and_then(|c| scores.iter().find(|s| s.country == c));
        let color = match score {
            Some(s) if s.matches => map_color(s.overall),
            Some(_) => INFEASIBLE,
            None => NO_ELIGIBLE_DATA,
        };
        let positions: Vec<_> = region.points.iter().map(|p| project(*p)).collect();
        let mesh = egui::Mesh {
            vertices: positions
                .iter()
                .map(|pos| egui::epaint::Vertex {
                    pos: *pos,
                    uv: egui::epaint::WHITE_UV,
                    color,
                })
                .collect(),
            indices: region.indices.clone(),
            ..Default::default()
        };
        if let Some(p) = pointer
            && region.contains(MapPoint {
                x: (p.x - view.left()) / view.width(),
                y: (p.y - view.top()) / view.height(),
            })
        {
            hovered = Some((&region.name, country));
        }
        painter.add(egui::Shape::mesh(mesh));
        for ring in &region.rings {
            painter.add(egui::Shape::closed_line(
                ring.iter().map(|i| positions[*i as usize]).collect(),
                Stroke::new(0.4, Color32::from_rgb(255, 255, 255)),
            ));
        }
    }
    // Markers keep tiny countries selectable even when 1:110m boundaries omit them.
    for s in scores {
        if let Some(location) = countries[s.country].location {
            let p = project(MapPoint {
                x: (location.longitude + 180.) / 360.,
                y: (90. - location.latitude) / 180.,
            });
            let is_selected = Some(s.country) == *selected;
            if is_selected {
                painter.circle_stroke(p, 7., Stroke::new(2., Color32::from_rgb(30, 41, 63)));
            }
            painter.circle_filled(
                p,
                if is_selected { 3.5 } else { 2. },
                if s.matches {
                    map_color(s.overall)
                } else {
                    Color32::from_rgb(133, 102, 87)
                },
            );
            if pointer.is_some_and(|mouse| mouse.distance(p) < 6.) {
                hovered = Some((&countries[s.country].name, Some(s.country)));
            }
        }
    }
    if let Some((name, country)) = hovered {
        let score = country.and_then(|c| scores.iter().find(|s| s.country == c));
        let text = match score {
            Some(s) if s.matches => format!(
                "{name}\nOverall {} · Rank {}\nClick to select",
                percent(s.overall),
                s.rank,
            ),
            Some(s) => format!("{name}\nOverall {}\nInfeasible", percent(s.overall)),
            None => format!("{name}\nInsufficient data for these settings"),
        };
        if response.clicked() && score.is_some_and(|s| s.matches) {
            *selected = country;
        }
        response.on_hover_text(text);
    }
    if let Some((min, max)) = range {
        let bar = egui::Rect::from_min_max(
            egui::pos2(rect.right() - 78., rect.top() + 28.),
            egui::pos2(rect.right() - 64., rect.bottom() - 62.),
        );
        painter.text(
            egui::pos2(bar.left(), rect.top() + 8.),
            egui::Align2::LEFT_TOP,
            "Overall",
            egui::FontId::proportional(12.),
            ui.visuals().text_color(),
        );
        let mut mesh = egui::Mesh::default();
        for i in 0..=64 {
            let t = i as f32 / 64.;
            let y = egui::lerp(bar.top()..=bar.bottom(), t);
            let color = score_color(if min == max { 0.5 } else { 1. - t as f64 });
            for x in [bar.left(), bar.right()] {
                mesh.vertices.push(egui::epaint::Vertex {
                    pos: egui::pos2(x, y),
                    uv: egui::epaint::WHITE_UV,
                    color,
                });
            }
            if i < 64 {
                let j = i * 2;
                mesh.indices
                    .extend_from_slice(&[j, j + 1, j + 2, j + 1, j + 3, j + 2]);
            }
        }
        painter.add(egui::Shape::mesh(mesh));
        let ticks: &[f32] = if min == max { &[0.5] } else { &[0., 0.5, 1.] };
        for &t in ticks {
            let y = egui::lerp(bar.top()..=bar.bottom(), t);
            painter.text(
                egui::pos2(bar.right() + 5., y),
                egui::Align2::LEFT_CENTER,
                percent(max - (max - min) * t as f64),
                egui::FontId::proportional(12.),
                ui.visuals().text_color(),
            );
        }
    }
    for (offset, color, label) in [
        (0., INFEASIBLE, "Infeasible"),
        (26., NO_ELIGIBLE_DATA, "Data missing"),
    ] {
        let pos = egui::pos2(rect.right() - 78., rect.bottom() - 49. + offset);
        painter.rect_filled(egui::Rect::from_min_size(pos, EVec2::splat(10.)), 2., color);
        painter.text(
            pos + EVec2::new(14., 0.),
            egui::Align2::LEFT_TOP,
            label,
            egui::FontId::proportional(10.),
            ui.visuals().text_color(),
        );
    }
    if let Some(country) = *selected {
        ui.label(RichText::new(format!("Selected: {}", countries[country].name)).color(ACCENT));
    }
}

fn visible_range(values: impl Iterator<Item = f64>) -> Option<(f64, f64)> {
    values.filter(|v| v.is_finite()).fold(None, |range, value| {
        Some(match range {
            Some((min, max)) => (value.min(min), value.max(max)),
            None => (value, value),
        })
    })
}

fn normalize(value: f64, range: Option<(f64, f64)>) -> f64 {
    match range {
        Some((min, max)) if max > min => (value - min) / (max - min),
        _ => 0.5,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colorbar_scales_to_data_and_handles_empty_or_constant_ranges() {
        let range = visible_range([0.42, 0.58, 0.50, f64::NAN].into_iter());
        assert_eq!(range, Some((0.42, 0.58)));
        assert_eq!(normalize(0.42, range), 0.);
        assert_eq!(normalize(0.58, range), 1.);
        assert_eq!(visible_range(std::iter::empty()), None);
        let constant = visible_range([0.6, 0.6].into_iter());
        assert_eq!(constant, Some((0.6, 0.6)));
        assert_eq!(normalize(0.6, constant), 0.5);
    }
}
