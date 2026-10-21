use crate::model::Quality;
use bevy_egui::egui::{self, Pos2, Rect, Vec2};

const LABEL: &str = "Quality of Life weights triangle";
const HELP: &str = "Drag the point to balance your preferences in 10% steps. Each corner is 100% of that preference. Lines parallel to the opposite edge have a constant share. Use arrow keys to move the point; Home restores equal shares.";

pub(super) fn show(ui: &mut egui::Ui, quality: &mut Quality) {
    let mobile = ui.ctx().content_rect().width() < 850.;
    let area_width = ui.available_width().min(300.);
    let width = if mobile {
        (area_width - 180.).max(80.)
    } else {
        area_width
    };
    let top = 42.;
    let padding = if mobile { 58. } else { 100. };
    let height = (width - 16.) * 3_f32.sqrt() / 2.;
    let (slot, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height + padding),
        egui::Sense::hover(),
    );
    let area = Rect::from_center_size(slot.center(), egui::vec2(area_width, height + padding));
    let plot = Rect::from_min_size(
        egui::pos2(area.center().x - width / 2., area.top() + top),
        egui::vec2(width, height + 16.),
    );
    let vertices = vertices(plot);
    let mut response = ui.interact(
        plot,
        ui.id().with("quality_triangle"),
        egui::Sense::click_and_drag(),
    );
    let before = *quality;
    let mut point = position(vertices, shares(*quality));
    if (response.clicked() || response.dragged())
        && let Some(pointer) = response.interact_pointer_pos()
    {
        point = pointer;
        response.request_focus();
        *quality = weights_at(vertices, point);
    }
    if response.has_focus() {
        ui.memory_mut(|memory| {
            memory.set_focus_lock_filter(
                response.id,
                egui::EventFilter {
                    horizontal_arrows: true,
                    vertical_arrows: true,
                    ..Default::default()
                },
            );
        });
        let delta = ui.input(|input| {
            let step = if input.modifiers.shift { 0.2 } else { 0.1 } * (width - 16.);
            Vec2::new(
                (input.num_presses(egui::Key::ArrowRight) as f32
                    - input.num_presses(egui::Key::ArrowLeft) as f32)
                    * step,
                (input.num_presses(egui::Key::ArrowDown) as f32
                    - input.num_presses(egui::Key::ArrowUp) as f32)
                    * step,
            )
        });
        if delta != Vec2::ZERO {
            *quality = weights_at(vertices, point + delta);
        }
        if ui.input(|input| input.key_pressed(egui::Key::Home)) {
            *quality = Quality {
                happy_planet: 5.,
                social_progress: 5.,
                human_freedom: 5.,
            };
        }
    }

    let colors = super::theme::quality_colors();
    let label_width = if mobile {
        90.
    } else {
        (area_width / 2.).min(140.)
    };
    let side_label_top = if mobile {
        vertices[1].y - 56.
    } else {
        plot.bottom()
    };
    let label_rects = [
        Rect::from_center_size(
            egui::pos2(area.center().x, area.top() + 20.),
            egui::vec2((area_width / 2.).min(140.), 40.),
        ),
        Rect::from_min_size(
            egui::pos2(area.left(), side_label_top),
            egui::vec2(label_width, if mobile { 48. } else { 40. }),
        ),
        Rect::from_min_size(
            egui::pos2(area.right() - label_width, side_label_top),
            egui::vec2(label_width, if mobile { 48. } else { 40. }),
        ),
    ];
    for (i, name) in super::theme::QUALITY_NAMES.iter().enumerate() {
        let corner = super::icons::preference(ui, label_rects[i], name, shares(*quality)[i]);
        if corner.clicked() {
            *quality = weights_at(vertices, vertices[i]);
        }
    }
    if *quality != before {
        response.mark_changed();
    }
    let shares = shares(*quality);
    response.widget_info(|| {
        let mut info = egui::WidgetInfo::labeled(egui::WidgetType::Other, ui.is_enabled(), LABEL);
        info.current_text_value = Some(format!(
            "Happy Planet {:.0}%, Social Progress {:.0}%, Human Freedom {:.0}%",
            shares[0] * 100.,
            shares[1] * 100.,
            shares[2] * 100.,
        ));
        info
    });
    let painter = ui.painter();
    painter.add(egui::Shape::convex_polygon(
        vertices.to_vec(),
        ui.visuals().extreme_bg_color,
        ui.visuals().widgets.noninteractive.bg_stroke,
    ));
    for i in 0..3 {
        for step in 1..10 {
            painter.line_segment(
                isoline(vertices, i, step as f32 / 10.),
                (0.5, ui.visuals().widgets.noninteractive.bg_stroke.color),
            );
        }
        painter.line_segment(isoline(vertices, i, shares[i] as f32), (1.5, colors[i]));
        painter.circle_filled(vertices[i], 3., colors[i]);
    }
    let point = position(vertices, shares);
    let visuals = ui.style().interact(&response);
    painter.circle(
        point,
        7.,
        ui.visuals().window_fill,
        (2., visuals.fg_stroke.color),
    );
    if response.has_focus() {
        painter.circle_stroke(point, 10., ui.visuals().selection.stroke);
    }
    response.on_hover_text(HELP);
}

fn vertices(rect: Rect) -> [Pos2; 3] {
    let rect = rect.shrink(8.);
    [rect.center_top(), rect.left_bottom(), rect.right_bottom()]
}

fn shares(quality: Quality) -> [f64; 3] {
    let values = quality.values();
    let total = values.iter().sum::<f64>();
    if total > 0. && values.iter().all(|v| v.is_finite() && *v >= 0.) {
        values.map(|v| v / total)
    } else {
        [1. / 3.; 3]
    }
}

fn position(vertices: [Pos2; 3], shares: [f64; 3]) -> Pos2 {
    vertices[0]
        + (vertices[1] - vertices[0]) * shares[1] as f32
        + (vertices[2] - vertices[0]) * shares[2] as f32
}

fn isoline(vertices: [Pos2; 3], index: usize, share: f32) -> [Pos2; 2] {
    [1, 2].map(|offset| vertices[(index + offset) % 3].lerp(vertices[index], share))
}

fn weights_at(vertices: [Pos2; 3], point: Pos2) -> Quality {
    // The 66 grid intersections keep every share in 10% steps and the total at 100%.
    (0..=10)
        .flat_map(|happy| {
            (0..=10 - happy).map(move |social| Quality {
                happy_planet: f64::from(happy),
                social_progress: f64::from(social),
                human_freedom: f64::from(10 - happy - social),
            })
        })
        .min_by(|a, b| {
            position(vertices, shares(*a))
                .distance_sq(point)
                .total_cmp(&position(vertices, shares(*b)).distance_sq(point))
        })
        .unwrap()
}

#[cfg(test)]
#[path = "quality_triangle_test.rs"]
mod tests;
