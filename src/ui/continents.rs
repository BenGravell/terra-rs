use crate::model::CONTINENTS;
use bevy_egui::egui;

// Natural Earth coastlines normalized to a 32 × 32 square, in CONTINENTS order.
#[allow(clippy::approx_constant)] // Geographic coordinates, not mathematical constants.
const OUTLINES: [&[&[[f32; 2]]]; 6] = include!("../../assets/continent_outlines.rs");

pub(super) fn show(ui: &mut egui::Ui, selected: &mut Vec<String>) {
    let gap = 6.;
    let width = ((ui.available_width() - gap) / 2.).floor();
    egui::Grid::new("continent_chips")
        .num_columns(2)
        .spacing(egui::vec2(gap, gap))
        .show(ui, |ui| {
            for (index, name) in CONTINENTS.into_iter().enumerate() {
                let active = selected.iter().any(|value| value == name);
                let (rect, response) =
                    ui.allocate_exact_size(egui::vec2(width, 64.), egui::Sense::click());
                response.widget_info(|| {
                    egui::WidgetInfo::selected(egui::WidgetType::Button, true, active, name)
                });
                let visuals = ui.style().interact_selectable(&response, active);
                ui.painter().rect(
                    rect,
                    3.,
                    visuals.bg_fill,
                    visuals.bg_stroke,
                    egui::StrokeKind::Inside,
                );
                let origin = egui::pos2(rect.center().x - 16., rect.top() + 4.);
                for ring in OUTLINES[index] {
                    let points = ring
                        .iter()
                        .map(|[x, y]| origin + egui::vec2(*x, *y))
                        .collect();
                    ui.painter().add(egui::Shape::line(
                        points,
                        egui::Stroke::new(1., visuals.text_color()),
                    ));
                }
                ui.painter().text(
                    egui::pos2(rect.center().x, rect.bottom() - 9.),
                    egui::Align2::CENTER_BOTTOM,
                    name,
                    egui::FontId::proportional(13.),
                    visuals.text_color(),
                );
                if response.clicked() {
                    if active {
                        selected.retain(|value| value != name);
                    } else {
                        selected.push(name.into());
                    }
                }
                if index % 2 == 1 {
                    ui.end_row();
                }
            }
        });
}

#[cfg(test)]
#[path = "continents_test.rs"]
mod tests;
