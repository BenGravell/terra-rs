use super::{
    bars,
    theme::{CULTURE, PARTS, QUALITY_NAMES, percent},
};
use crate::model::{Country, Score};
use bevy_egui::egui;

const FLAG_IDS: &str = include_str!("../../assets/flags/countries.txt");
const FLAG_PIXELS: &[u8] = include_bytes!("../../assets/flags/atlas.rgba");

pub(super) fn show(
    ui: &mut egui::Ui,
    countries: &[Country],
    scores: &[Score],
    selected: &mut Option<usize>,
    quality_weight: f64,
) {
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
            ui.colored_label(CULTURE, "■ Culture Fit");
            for (name, color) in QUALITY_NAMES.iter().zip(PARTS) {
                ui.colored_label(color, format!("■ {name}"));
            }
        });
        let flag_texture_id = egui::Id::new("country_flags");
        let flags = ui
            .ctx()
            .data_mut(|data| data.get_temp::<egui::TextureHandle>(flag_texture_id))
            .unwrap_or_else(|| {
                let texture = ui.ctx().load_texture(
                    "country_flags",
                    egui::ColorImage::from_rgba_unmultiplied(
                        [32, FLAG_PIXELS.len() / (32 * 4)],
                        FLAG_PIXELS,
                    ),
                    egui::TextureOptions::LINEAR,
                );
                ui.ctx()
                    .data_mut(|data| data.insert_temp(flag_texture_id, texture.clone()));
                texture
            });
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
        let table_height = if compact {
            (ui.available_height() - 34.).clamp(100., 340.)
        } else {
            340.
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
                            let bar = bars::stacked_bar(
                                ui,
                                egui::vec2(bar_width, if compact { 12. } else { 16. }),
                                &values,
                                &[CULTURE, PARTS[0], PARTS[1], PARTS[2]],
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
    let atlas_height = (FLAG_PIXELS.len() / (32 * 4)) as f32;
    height * (uv.width() * 32.) / (uv.height() * atlas_height)
}

fn flag_uv(index: usize) -> egui::Rect {
    let rows = &FLAG_PIXELS[index * 32 * 24 * 4..(index + 1) * 32 * 24 * 4];
    let visible = |row: &[u8]| row.chunks_exact(4).any(|pixel| pixel[3] != 0);
    let top = rows.chunks_exact(32 * 4).position(visible).unwrap_or(0);
    let bottom = 24
        - rows
            .chunks_exact(32 * 4)
            .rev()
            .position(visible)
            .unwrap_or(0);
    let height = (FLAG_PIXELS.len() / (32 * 4)) as f32;
    egui::Rect::from_min_max(
        egui::pos2(0., (index * 24 + top) as f32 / height),
        egui::pos2(1., (index * 24 + bottom) as f32 / height),
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
