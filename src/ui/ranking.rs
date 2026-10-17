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
        let compact = ui.ctx().content_rect().width() < 850. || ui.available_width() < 600.;
        if compact {
            ui.style_mut()
                .text_styles
                .insert(egui::TextStyle::Body, egui::FontId::proportional(13.));
            ui.spacing_mut().item_spacing.x = 6.;
            ui.spacing_mut().item_spacing.y = 2.;
            ui.spacing_mut().interact_size.y = 18.;
        }
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
        let column_gap = if compact { 6. } else { 16. };
        let rank_diameter = ui.text_style_height(&egui::TextStyle::Body) + 6.;
        let rank_width = rank_diameter;
        let score_width = text_width("100%");
        let minimum_bar_width = if compact { 60. } else { 180. };
        let available = (ui.available_width() - ui.spacing().scroll.allocated_width()).floor();
        let fixed_width = rank_width + score_width + column_gap * 3.;
        let country_width = (available * 0.35)
            .min(200.)
            .min(available - fixed_width - minimum_bar_width)
            .max(flag_slot_width + ui.spacing().item_spacing.x);
        let bar_width = (available - fixed_width - country_width).max(minimum_bar_width);
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.;
            ui.add_space(fixed_width + country_width);
            ui.allocate_ui_with_layout(
                egui::vec2(bar_width, 18.),
                egui::Layout::left_to_right(egui::Align::Center),
                |ui| {
                    ui.set_width(bar_width);
                    ui.spacing_mut().item_spacing.x = 8.;
                    let mut legend_font = egui::TextStyle::Body.resolve(ui.style());
                    let legend_width = ui.available_width() - 3. * ui.spacing().item_spacing.x;
                    while std::iter::once("Culture Fit")
                        .chain(QUALITY_NAMES)
                        .map(|name| {
                            ui.painter()
                                .layout_no_wrap(
                                    format!("■ {name}"),
                                    legend_font.clone(),
                                    ui.visuals().text_color(),
                                )
                                .size()
                                .x
                        })
                        .sum::<f32>()
                        > legend_width
                        && legend_font.size > 1.
                    {
                        legend_font.size -= 0.25;
                    }
                    ui.horizontal(|ui| {
                        for (name, color) in std::iter::once(("Culture Fit", CULTURE))
                            .chain(QUALITY_NAMES.into_iter().zip(parts))
                        {
                            let mut label = egui::text::LayoutJob::default();
                            let font = legend_font.clone();
                            label.append("■ ", 0., egui::TextFormat::simple(font.clone(), color));
                            label.append(
                                name,
                                0.,
                                egui::TextFormat::simple(font, ui.visuals().text_color()),
                            );
                            ui.add(egui::Label::new(label).wrap_mode(egui::TextWrapMode::Extend));
                        }
                    });
                },
            );
        });
        let table_height = ui.available_height().max(0.);
        egui::ScrollArea::vertical()
            .id_salt("ranking")
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
            .max_height(table_height)
            .auto_shrink([false, false])
            .show(ui, |ui| {
                egui::Grid::new("country_ranking_table")
                    .num_columns(4)
                    .min_col_width(0.)
                    .spacing(egui::vec2(column_gap, if compact { 2. } else { 8. }))
                    .min_row_height(18.)
                    .striped(true)
                    .show(ui, |ui| {
                        for s in scores.iter().filter(|s| s.matches) {
                            let row_top = ui.cursor().top();
                            let background = ui.painter().add(egui::Shape::Noop);
                            let rank = super::disk::show(
                                ui,
                                &s.rank.to_string(),
                                rank_diameter,
                                egui::Sense::hover(),
                            );
                            let country = &countries[s.country];
                            let country_cell = ui.allocate_ui_with_layout(
                                egui::vec2(country_width, capitals.size().y),
                                egui::Layout::left_to_right(egui::Align::Center),
                                |ui| {
                                    ui.set_width(country_width);
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
                            let overall = ui.add_sized(
                                [score_width, 18.],
                                egui::Label::new(percent(s.overall))
                                    .halign(egui::Align::Min)
                                    .selectable(false),
                            );
                            let values = contributions(s, quality_weight);
                            let row_height = rank
                                .rect
                                .height()
                                .max(country_cell.response.rect.height())
                                .max(overall.rect.height())
                                .max(18.);
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
                            row_rect.max.y = row_rect.bottom().max(row_top + 18.);
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
