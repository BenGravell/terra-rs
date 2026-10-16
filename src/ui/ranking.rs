use super::{
    bars,
    theme::{CULTURE, QUALITY_NAMES, percent, quality_colors},
};
use crate::model::{Country, Score};
use bevy_egui::egui;

use super::flags::{FLAG_IDS, FLAG_PIXELS};

pub(super) fn show(
    ui: &mut egui::Ui,
    countries: &[Country],
    scores: &[Score],
    selected: &mut Option<usize>,
    quality_weight: f64,
) {
    let parts = quality_colors();
    ui.scope(|ui| {
        ui.spacing_mut().scroll.floating = false;
        let compact = ui.available_width() < 600.;
        if compact {
            ui.style_mut()
                .text_styles
                .insert(egui::TextStyle::Body, egui::FontId::proportional(13.));
            ui.spacing_mut().item_spacing.x = 6.;
        }
        ui.horizontal_wrapped(|ui| {
            for (name, color) in std::iter::once(("Culture Fit", CULTURE))
                .chain(QUALITY_NAMES.into_iter().zip(parts))
            {
                let mut label = egui::text::LayoutJob::default();
                let font = egui::TextStyle::Body.resolve(ui.style());
                label.append("■ ", 0., egui::TextFormat::simple(font.clone(), color));
                label.append(
                    name,
                    0.,
                    egui::TextFormat::simple(font, ui.visuals().text_color()),
                );
                ui.label(label);
            }
        });
        let flags = super::flags::texture(ui);
        let capitals = ui.painter().layout_no_wrap(
            "H".into(),
            egui::TextStyle::Body.resolve(ui.style()),
            ui.visuals().text_color(),
        );
        let cap_bounds = capitals.rows[0].visuals.mesh_bounds;
        let flag_slot_width = (0..FLAG_IDS.lines().count())
            .map(|index| flag_width(flag_uv(index), cap_bounds.height()))
            .fold(0., f32::max);
        let font = egui::TextStyle::Body.resolve(ui.style());
        let text_width = |text: &str| {
            ui.painter()
                .layout_no_wrap(text.into(), font.clone(), ui.visuals().text_color())
                .size()
                .x
        };
        let headings = if compact {
            ["#", "Country", "Score", "Breakdown"]
        } else {
            ["Rank", "Country", "Overall", "Score breakdown"]
        };
        let column_gap = if compact { 6. } else { 16. };
        let rank_width = text_width(headings[0]).max(text_width(&format!(
            "{}.",
            scores.iter().filter(|s| s.matches).count()
        )));
        let score_width = text_width(headings[2]).max(text_width("100%"));
        let available = ui.available_width() - ui.spacing().scroll.allocated_width();
        let minimum_bar_width = text_width(headings[3]).max(if compact { 60. } else { 180. });
        let country_text_width = scores
            .iter()
            .filter(|score| score.matches)
            .map(|score| text_width(&countries[score.country].name))
            .fold(text_width(headings[1]), f32::max);
        let fixed_width = rank_width + score_width + column_gap * 3.;
        let country_width = (flag_slot_width + ui.spacing().item_spacing.x + country_text_width)
            .min(available - fixed_width - minimum_bar_width)
            .max(flag_slot_width + ui.spacing().item_spacing.x);
        let bar_width = (available - fixed_width - country_width).max(minimum_bar_width);
        let table_height = if ui.ctx().content_rect().width() < 850. {
            (ui.available_height() - 34.).clamp(100., 340.)
        } else {
            ui.available_height().max(0.)
        };
        egui::ScrollArea::both()
            .id_salt("ranking")
            .max_height(table_height)
            .auto_shrink([false, true])
            .show(ui, |ui| {
                egui::Grid::new("country_ranking_table")
                    .num_columns(4)
                    .min_col_width(0.)
                    .spacing(egui::vec2(column_gap, 8.))
                    .min_row_height(if compact { 28. } else { 18. })
                    .striped(true)
                    .show(ui, |ui| {
                        for heading in headings {
                            ui.strong(heading);
                        }
                        ui.end_row();
                        for s in scores.iter().filter(|s| s.matches) {
                            let row_top = ui.cursor().top();
                            let background = ui.painter().add(egui::Shape::Noop);
                            let rank =
                                ui.add(egui::Label::new(format!("{}.", s.rank)).selectable(false));
                            let country = &countries[s.country];
                            let country_cell = ui.allocate_ui_with_layout(
                                egui::vec2(country_width, capitals.size().y),
                                egui::Layout::left_to_right(egui::Align::Center),
                                |ui| {
                                    ui.set_min_width(country_width);
                                    let (slot, _) = ui.allocate_exact_size(
                                        egui::vec2(flag_slot_width, capitals.size().y),
                                        egui::Sense::hover(),
                                    );
                                    let name = ui
                                        .add(
                                            egui::Label::new(&country.name)
                                                .selectable(false)
                                                .truncate(),
                                        )
                                        .on_hover_text(&country.name);
                                    if let Some(index) =
                                        FLAG_IDS.lines().position(|id| id == country.id)
                                    {
                                        let uv = flag_uv(index);
                                        let width = flag_width(uv, cap_bounds.height());
                                        let rect = egui::Rect::from_min_size(
                                            egui::pos2(
                                                slot.left(),
                                                name.rect.top() + cap_bounds.top(),
                                            ),
                                            egui::vec2(width, cap_bounds.height()),
                                        );
                                        ui.painter().image(
                                            flags.id(),
                                            rect,
                                            uv,
                                            egui::Color32::WHITE,
                                        );
                                    }
                                },
                            );
                            let overall =
                                ui.add(egui::Label::new(percent(s.overall)).selectable(false));
                            let values = contributions(s, quality_weight);
                            let row_height = rank
                                .rect
                                .height()
                                .max(country_cell.response.rect.height())
                                .max(overall.rect.height())
                                .max(if compact { 28. } else { 18. });
                            let bar = bars::stacked_bar(
                                ui,
                                egui::vec2(bar_width, row_height),
                                &values,
                                &[CULTURE, parts[0], parts[1], parts[2]],
                                *selected == Some(s.country),
                                &[
                                    "Culture Fit",
                                    QUALITY_NAMES[0],
                                    QUALITY_NAMES[1],
                                    QUALITY_NAMES[2],
                                ],
                            );
                            let mut row_rect = rank
                                .rect
                                .union(country_cell.response.rect)
                                .union(overall.rect)
                                .union(bar.rect);
                            row_rect.min.y = row_top;
                            row_rect.max.y = row_rect
                                .bottom()
                                .max(row_top + if compact { 28. } else { 18. });
                            let row_rect =
                                row_rect.expand2(egui::vec2(0., ui.spacing().item_spacing.y * 0.5));
                            let response = ui.interact(
                                row_rect,
                                ui.id().with(("country_row", s.country)),
                                egui::Sense::click(),
                            );
                            response.widget_info(|| {
                                egui::WidgetInfo::selected(
                                    egui::WidgetType::SelectableLabel,
                                    true,
                                    *selected == Some(s.country),
                                    format!("Select {}", country.name),
                                )
                            });
                            if response.clicked() {
                                *selected = Some(s.country);
                            }
                            if *selected == Some(s.country)
                                || response.hovered()
                                || response.has_focus()
                            {
                                let fill = if *selected == Some(s.country) {
                                    ui.visuals().selection.bg_fill
                                } else {
                                    ui.visuals().widgets.hovered.bg_fill
                                };
                                ui.painter()
                                    .set(background, egui::Shape::rect_filled(row_rect, 2., fill));
                            }
                            ui.end_row();
                        }
                    });
            });
    });
}

fn flag_width(uv: egui::Rect, height: f32) -> f32 {
    let atlas_height = (FLAG_IDS.lines().count().div_ceil(2) * 24) as f32;
    height * (uv.width() * 64.) / (uv.height() * atlas_height)
}

fn flag_uv(index: usize) -> egui::Rect {
    let rows = &FLAG_PIXELS[index * 32 * 24 * 4..(index + 1) * 32 * 24 * 4];
    let rows = rows.as_chunks::<{ 32 * 4 }>().0;
    let visible = |row: &[u8; 32 * 4]| row.as_chunks::<4>().0.iter().any(|pixel| pixel[3] != 0);
    let top = rows.iter().position(visible).unwrap_or(0);
    let bottom = 24 - rows.iter().rev().position(visible).unwrap_or(0);
    let height = (FLAG_IDS.lines().count().div_ceil(2) * 24) as f32;
    let cell = super::flags::cell_uv(index);
    egui::Rect::from_min_max(
        egui::pos2(cell.left(), cell.top() + top as f32 / height),
        egui::pos2(cell.right(), cell.top() + bottom as f32 / height),
    )
}

fn contributions(score: &Score, quality_weight: f64) -> [f64; 4] {
    let quality = score
        .quality_contributions
        .values()
        .map(|value| value * quality_weight);
    [
        score.overall_contributions.culture,
        quality[0],
        quality[1],
        quality[2],
    ]
}

#[cfg(test)]
#[path = "ranking_test.rs"]
mod tests;
