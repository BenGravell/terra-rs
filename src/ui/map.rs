use super::theme::{
    ACCENT, BORDER, INFEASIBLE, MAP_BACKGROUND, NO_ELIGIBLE_DATA, SURFACE, percent, score_color,
};
use crate::model::{Country, MapPoint, MapRegion, Score};
use bevy_egui::egui::{self, RichText, Stroke, Vec2 as EVec2};

pub(super) fn show(
    ui: &mut egui::Ui,
    countries: &[Country],
    regions: &[MapRegion],
    scores: &[Score],
    selected: &mut Option<usize>,
    max_height: f32,
) {
    ui.add_space(8.);
    let width = ui.available_width();
    let map_area_width = (width - 92.).max(1.);
    let height = (map_area_width / 2.).max(100.).min(max_height.max(100.));
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
    let (rect, _) = ui.allocate_exact_size(EVec2::new(width, height), egui::Sense::hover());
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 6., MAP_BACKGROUND);
    let map_width = (height * 2.).min(map_area_width);
    let base_view = egui::Rect::from_center_size(
        egui::pos2(rect.left() + map_area_width / 2., rect.center().y),
        EVec2::new(map_width, map_width / 2.),
    );
    let viewport =
        egui::Rect::from_min_max(rect.min, egui::pos2(rect.right() - 92., rect.bottom()));
    let response = ui.interact(
        viewport,
        ui.id().with("map_canvas"),
        egui::Sense::click_and_drag(),
    );
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Other, true, "Country map"));
    let state_id = egui::Id::new("map_navigation");
    let mut navigation = ui
        .ctx()
        .data_mut(|data| data.get_temp::<Navigation>(state_id).unwrap_or_default());
    let touch = ui.input(|i| i.multi_touch());
    if ui.input(|i| i.pointer.any_pressed()) && touch.is_none() {
        navigation.gesturing = false;
    }
    if let Some(touch) = touch {
        if navigation.gesturing || response.is_pointer_button_down_on() {
            navigation.gesturing = true;
            navigation.zoom_at(
                touch.zoom_delta,
                touch.center_pos - touch.translation_delta,
                base_view,
            );
            navigation.pan(touch.translation_delta, base_view);
        }
    } else {
        if response.dragged() {
            navigation.pan(response.drag_delta(), base_view);
        }
        if response.hovered() {
            let (zoom, scroll, pointer) = ui.input(|i| {
                (
                    i.zoom_delta(),
                    i.smooth_scroll_delta(),
                    i.pointer.hover_pos(),
                )
            });
            if let Some(pointer) = pointer {
                navigation.zoom_at(zoom * (scroll.y * 0.005).exp(), pointer, base_view);
                navigation.pan(egui::vec2(scroll.x, 0.), base_view);
                // Keep the surrounding results scroll area still while navigating the map.
                ui.input_mut(|i| i.smooth_scroll_delta = EVec2::ZERO);
            }
        }
    }
    let view = navigation.view(base_view);
    let painter = painter.with_clip_rect(viewport.intersect(ui.clip_rect()));
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
            Stroke::new(0.5, BORDER),
        );
    }
    for y in 1..4 {
        let y = view.top() + view.height() * y as f32 / 4.;
        painter.line_segment(
            [egui::pos2(view.left(), y), egui::pos2(view.right(), y)],
            Stroke::new(0.5, BORDER),
        );
    }
    // Touch release removes hover, but the response retains the interaction position.
    let pointer = response
        .interact_pointer_pos()
        .or_else(|| response.hover_pos())
        .filter(|p| viewport.contains(*p));
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
            // Separate edges avoid unbounded miter joins at near-reversing coastlines.
            for (a, b) in ring
                .iter()
                .zip(ring.iter().cycle().skip(1))
                .take(ring.len())
            {
                painter.line_segment(
                    [positions[*a as usize], positions[*b as usize]],
                    Stroke::new(0.4, SURFACE),
                );
            }
        }
    }
    let mut selected_marker = None;
    // Markers keep tiny countries selectable even when 1:110m boundaries omit them.
    for s in scores {
        if let Some(location) = countries[s.country].location {
            let p = project(MapPoint {
                x: (location.longitude + 180.) / 360.,
                y: (90. - location.latitude) / 180.,
            });
            let is_selected = Some(s.country) == *selected;
            if is_selected {
                selected_marker = Some(p);
            }
            painter.circle_filled(
                p,
                if is_selected { 3.5 } else { 2. },
                if s.matches {
                    map_color(s.overall)
                } else {
                    INFEASIBLE
                },
            );
            if pointer.is_some_and(|mouse| mouse.distance(p) < 6.) {
                hovered = Some((&countries[s.country].name, Some(s.country)));
            }
        }
    }
    if let Some(p) = selected_marker {
        // Draw last so nearby country dots cannot obscure the selection.
        painter.circle_filled(p, 7., egui::Color32::BLACK);
        painter.circle_filled(p, 5., egui::Color32::WHITE);
        painter.circle_filled(p, 2.5, egui::Color32::BLACK);
    }
    if let Some((name, country)) = hovered {
        let score = country.and_then(|c| scores.iter().find(|s| s.country == c));
        let text = match score {
            Some(s) if s.matches => format!(
                "{name}\nOverall {} · Rank {}\nTap or click to select",
                percent(s.overall),
                s.rank,
            ),
            Some(s) => format!("{name}\nOverall {}\nInfeasible", percent(s.overall)),
            None => format!("{name}\nInsufficient data for these settings"),
        };
        if response.clicked() && !navigation.gesturing && score.is_some_and(|s| s.matches) {
            *selected = country;
        }
        response.on_hover_text(text);
    }
    let painter = ui.painter_at(rect);
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
    ui.ctx()
        .data_mut(|data| data.insert_temp(state_id, navigation));
    if let Some(country) = *selected {
        ui.label(RichText::new(format!("Selected: {}", countries[country].name)).color(ACCENT));
    }
}

#[derive(Clone, Copy)]
struct Navigation {
    zoom: f32,
    center: egui::Pos2,
    gesturing: bool,
}

impl Default for Navigation {
    fn default() -> Self {
        Self {
            zoom: 1.,
            center: egui::pos2(0.5, 0.5),
            gesturing: false,
        }
    }
}

impl Navigation {
    fn view(self, base: egui::Rect) -> egui::Rect {
        egui::Rect::from_min_size(
            base.center() - self.center.to_vec2() * base.size() * self.zoom,
            base.size() * self.zoom,
        )
    }

    fn pan(&mut self, delta: EVec2, base: egui::Rect) {
        self.center -= delta / (base.size() * self.zoom);
        let edge = 0.5 / self.zoom;
        self.center.x = self.center.x.clamp(edge, 1. - edge);
        self.center.y = self.center.y.clamp(edge, 1. - edge);
    }

    fn zoom_at(&mut self, factor: f32, anchor: egui::Pos2, base: egui::Rect) {
        let before = self.zoom;
        self.zoom = (self.zoom * factor).clamp(1., 16.);
        self.center += (anchor - base.center()) / base.size() * (1. / before - 1. / self.zoom);
        self.pan(EVec2::ZERO, base);
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

    use egui_kittest::{Harness, kittest::Queryable};

    fn map_harness(width: f32) -> Harness<'static, Option<usize>> {
        let data =
            crate::model::Dataset::parse(include_str!("../../assets/countries.json")).unwrap();
        let scores =
            crate::scoring::evaluate(&data.countries, &crate::model::Settings::default()).unwrap();
        let country = scores.iter().find(|s| s.matches).unwrap().country;
        let region = MapRegion {
            id: data.countries[country].id.clone(),
            name: data.countries[country].name.clone(),
            points: vec![
                MapPoint { x: 0.25, y: 0.25 },
                MapPoint { x: 0.75, y: 0.25 },
                MapPoint { x: 0.5, y: 0.75 },
            ],
            indices: vec![0, 1, 2],
            rings: vec![vec![0, 1, 2]],
        };
        Harness::builder()
            .with_size(egui::vec2(width, 500.))
            .build_ui_state(
                move |ui, selected| {
                    egui_extras::install_image_loaders(ui.ctx());
                    show(
                        ui,
                        &data.countries,
                        std::slice::from_ref(&region),
                        &scores,
                        selected,
                        320.,
                    )
                },
                None,
            )
    }

    fn pointer(h: &mut Harness<'_, Option<usize>>, pos: egui::Pos2, pressed: bool) {
        h.event(egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        });
    }

    fn touch(
        h: &mut Harness<'_, Option<usize>>,
        id: u64,
        pos: egui::Pos2,
        phase: egui::TouchPhase,
    ) {
        h.event(egui::Event::Touch {
            device_id: egui::TouchDeviceId(0),
            id: egui::TouchId(id),
            phase,
            pos,
            force: None,
        });
    }

    fn navigation(h: &Harness<'_, Option<usize>>) -> Navigation {
        h.ctx
            .data_mut(|data| data.get_temp(egui::Id::new("map_navigation")).unwrap())
    }

    #[test]
    fn touch_release_selects_without_hover() {
        for width in [320., 390., 1100.] {
            let mut h = map_harness(width);
            h.run();
            let map = h.get_by_label("Country map").rect();
            let pos = egui::pos2(
                map.center().x + map.width().min(640.) * 0.20,
                map.center().y - map.width().min(640.) * 0.10,
            );
            touch(&mut h, 0, pos, egui::TouchPhase::Start);
            h.event(egui::Event::PointerMoved(pos));
            pointer(&mut h, pos, true);
            h.step();
            h.input_mut().events.extend([
                egui::Event::Touch {
                    device_id: egui::TouchDeviceId(0),
                    id: egui::TouchId(0),
                    phase: egui::TouchPhase::End,
                    pos,
                    force: None,
                },
                egui::Event::PointerButton {
                    pos,
                    button: egui::PointerButton::Primary,
                    pressed: false,
                    modifiers: egui::Modifiers::NONE,
                },
                egui::Event::PointerGone,
            ]);
            h.run();
            assert!(h.state().is_some(), "tap failed at width {width}");
        }
    }

    #[test]
    fn pinch_and_drag_navigate_without_selecting() {
        let mut h = map_harness(1100.);
        h.run();
        let center = h.get_by_label("Country map").rect().center();
        let left = center - egui::vec2(20., 0.);
        let right = center + egui::vec2(20., 0.);
        h.event(egui::Event::PointerMoved(left));
        pointer(&mut h, left, true);
        touch(&mut h, 0, left, egui::TouchPhase::Start);
        touch(&mut h, 1, right, egui::TouchPhase::Start);
        h.step();
        touch(
            &mut h,
            0,
            left - egui::vec2(20., 0.),
            egui::TouchPhase::Move,
        );
        touch(
            &mut h,
            1,
            right + egui::vec2(20., 0.),
            egui::TouchPhase::Move,
        );
        h.step();
        assert!(
            (navigation(&h).zoom - 2.).abs() < 0.01,
            "zoom {}",
            navigation(&h).zoom
        );
        touch(&mut h, 0, left, egui::TouchPhase::End);
        touch(&mut h, 1, right, egui::TouchPhase::End);
        pointer(&mut h, left, false);
        h.event(egui::Event::PointerGone);
        h.run();
        assert_eq!(*h.state(), None);
        let before = navigation(&h).center;
        h.hover_at(center);
        h.drag_at(center);
        h.hover_at(center + egui::vec2(20., 10.));
        h.drop_at(center + egui::vec2(20., 10.));
        h.run();
        assert_ne!(navigation(&h).center, before);
        assert_eq!(*h.state(), None);
    }

    #[test]
    fn mouse_wheel_zooms_only_over_map() {
        let mut h = map_harness(1100.);
        h.run_steps(30);
        let map = h.get_by_label("Country map").rect();
        let wheel = |h: &Harness<'_, Option<usize>>, delta| {
            h.event(egui::Event::MouseWheel {
                unit: egui::MouseWheelUnit::Point,
                phase: egui::TouchPhase::Move,
                delta: egui::vec2(0., delta),
                modifiers: egui::Modifiers::NONE,
            });
        };
        h.hover_at(map.center() + egui::vec2(40., 0.));
        wheel(&h, 100.);
        h.run_steps(30);
        let zoomed = navigation(&h).zoom;
        assert!(zoomed > 1.);
        assert!(navigation(&h).center.x > 0.5);
        assert_eq!(*h.state(), None);
        wheel(&h, -50.);
        h.run_steps(30);
        assert!(navigation(&h).zoom < zoomed);
        let before = navigation(&h).zoom;
        h.hover_at(egui::pos2(map.right() + 40., map.center().y));
        wheel(&h, 100.);
        h.run_steps(30);
        assert_eq!(navigation(&h).zoom, before);
    }

    #[test]
    fn zoom_keeps_anchor_fixed_and_navigation_stays_bounded() {
        let base = egui::Rect::from_min_size(egui::pos2(20., 30.), egui::vec2(400., 200.));
        let anchor = base.center() + egui::vec2(50., 20.);
        let mut nav = Navigation::default();
        let point = (anchor - nav.view(base).min) / nav.view(base).size();
        nav.zoom_at(3., anchor, base);
        assert!((nav.view(base).min + point * nav.view(base).size()).distance(anchor) < 0.001);
        nav.pan(egui::vec2(10000., -10000.), base);
        assert!(nav.view(base).expand(0.01).contains_rect(base));
        nav.zoom_at(1000., anchor, base);
        assert_eq!(nav.zoom, 16.);
        nav.zoom_at(0.001, anchor, base);
        assert_eq!(nav.zoom, 1.);
        assert_eq!(nav.view(base), base);
    }

    #[test]
    fn country_outlines_do_not_generate_spikes_outside_the_map() {
        let regions = MapRegion::parse(include_str!("../../assets/map.json")).unwrap();
        for size in [egui::vec2(1100., 500.), egui::vec2(700., 400.)] {
            let ctx = egui::Context::default();
            let output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
                    ..Default::default()
                },
                |ui| {
                    egui::CentralPanel::default().show(ui, |ui| {
                        show(ui, &[], &regions, &[], &mut None, 320.);
                    });
                },
            );
            for primitive in ctx.tessellate(output.shapes, output.pixels_per_point) {
                if let egui::epaint::Primitive::Mesh(mesh) = primitive.primitive {
                    for vertex in mesh.vertices {
                        assert!(
                            primitive.clip_rect.expand(2.).contains(vertex.pos),
                            "Outline spike at {:?}, outside {:?}",
                            vertex.pos,
                            primitive.clip_rect
                        );
                    }
                }
            }
        }
    }

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
