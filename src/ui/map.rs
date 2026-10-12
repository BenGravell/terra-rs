use super::theme::{ACCENT, percent, score_color};
use crate::model::{Country, MapPoint, MapRegion, Score};
use bevy_egui::egui::{self, Color32, RichText, Stroke, Vec2 as EVec2};

pub(super) fn show(
    ui: &mut egui::Ui,
    countries: &[Country],
    regions: &[MapRegion],
    scores: &[Score],
    selected: &mut Option<usize>,
) {
    ui.add_space(8.);
    let width = ui.available_width();
    let height = (width / 2.).min(340.);
    let (rect, response) = ui.allocate_exact_size(EVec2::new(width, height), egui::Sense::click());
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 6., Color32::from_rgb(10, 23, 36));
    let map_width = (height * 2.).min(width);
    let view = egui::Rect::from_center_size(rect.center(), EVec2::new(map_width, height));
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
            Stroke::new(0.5, Color32::from_rgb(31, 47, 61)),
        );
    }
    for y in 1..4 {
        let y = view.top() + view.height() * y as f32 / 4.;
        painter.line_segment(
            [egui::pos2(view.left(), y), egui::pos2(view.right(), y)],
            Stroke::new(0.5, Color32::from_rgb(31, 47, 61)),
        );
    }
    let pointer = response.hover_pos();
    let mut hovered: Option<(&str, Option<usize>)> = None;
    for region in regions {
        let country = countries.iter().position(|c| c.id == region.id);
        let score = country.and_then(|c| scores.iter().find(|s| s.country == c));
        let color = match score {
            Some(s) if s.matches => score_color(s.overall),
            Some(_) => Color32::from_rgb(78, 67, 65),
            None => Color32::from_rgb(43, 55, 69),
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
                Stroke::new(0.4, Color32::from_rgb(36, 58, 66)),
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
                painter.circle_stroke(p, 7., Stroke::new(2., Color32::WHITE));
            }
            painter.circle_filled(
                p,
                if is_selected { 3.5 } else { 2. },
                if s.matches {
                    score_color(s.overall)
                } else {
                    Color32::from_rgb(142, 112, 100)
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
            Some(s) => format!(
                "{name}\nOverall {} · Rank {}\n{}",
                percent(s.overall),
                s.rank,
                if s.matches {
                    "Click to select"
                } else {
                    "Outside your constraints"
                }
            ),
            None => format!("{name}\nInsufficient data for these settings"),
        };
        if response.clicked() && score.is_some_and(|s| s.matches) {
            *selected = country;
        }
        response.on_hover_text(text);
    }
    ui.horizontal_wrapped(|ui| {
        ui.small("Overall score");
        for value in [0., 0.25, 0.5, 0.75, 1.] {
            ui.horizontal(|ui| {
                ui.colored_label(score_color(value), "■");
                ui.small(format!("{:.0}%", value * 100.));
            });
        }
        ui.small("Brown: outside constraints · Grey: no eligible data");
    });
    if let Some(country) = *selected {
        ui.label(RichText::new(format!("Selected: {}", countries[country].name)).color(ACCENT));
        if let Some(s) = scores.iter().find(|s| s.country == country) {
            ui.small(format!(
                "Overall {} · Culture Fit {} · Quality of Life {}",
                percent(s.overall),
                percent(s.culture),
                percent(s.quality)
            ));
        }
    }
}
