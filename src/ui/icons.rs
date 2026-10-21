use bevy_egui::egui;

pub(super) fn tab(label: &str, color: egui::Color32, size: f32) -> egui::Image<'static> {
    let source = match label {
        "Table" | "Results" => egui::include_image!("../../assets/icons/lucide/table.svg"),
        "Map" => egui::include_image!("../../assets/icons/lucide/map.svg"),
        "About" => egui::include_image!("../../assets/icons/lucide/info.svg"),
        "Culture" => egui::include_image!("../../assets/icons/lucide/users.svg"),
        "Weights" => egui::include_image!("../../assets/icons/lucide/sliders-horizontal.svg"),
        "Filters" => egui::include_image!("../../assets/icons/lucide/list-filter.svg"),
        "Presets" => egui::include_image!("../../assets/icons/lucide/bookmark.svg"),
        _ => unreachable!("unknown tab: {label}"),
    };
    egui::Image::new(source)
        .fit_to_exact_size(egui::Vec2::splat(size))
        .tint(color)
}

pub(super) fn section(label: &'static str, default_open: bool) -> egui::CollapsingHeader {
    egui::CollapsingHeader::new(label)
        .default_open(default_open)
        .icon(move |ui, _, response| {
            let color = ui.style().interact(response).text_color();
            tab(label, color, 16.).paint_at(
                ui,
                egui::Rect::from_center_size(response.rect.center(), egui::Vec2::splat(16.)),
            );
        })
}

fn preference_icon(label: &str, color: egui::Color32) -> egui::Image<'static> {
    let source = match label {
        "Culture Fit" => egui::include_image!("../../assets/icons/lucide/users.svg"),
        "Quality of Life" => egui::include_image!("../../assets/icons/quality/quality-of-life.svg"),
        "Happy Planet" => egui::include_image!("../../assets/icons/quality/happy-planet.svg"),
        "Social Progress" => egui::include_image!("../../assets/icons/quality/social-progress.svg"),
        "Human Freedom" => egui::include_image!("../../assets/icons/quality/human-freedom.svg"),
        _ => unreachable!("unknown preference: {label}"),
    };
    egui::Image::new(source)
        .fit_to_exact_size(egui::Vec2::splat(32.))
        .tint(color)
}

pub(super) fn preference(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    name: &str,
    share: f64,
) -> egui::Response {
    let compact = rect.width() < 120.;
    let label = if compact {
        name.replace(' ', "\n")
    } else {
        name.to_owned()
    };
    let text = format!("{label}\n{:.0}%", share * 100.);
    let icon = preference_icon(name, ui.visuals().text_color())
        .fit_to_exact_size(egui::Vec2::splat(if compact { 16. } else { 32. }));
    let response = ui
        .put(
            rect,
            egui::Button::new((icon, egui::RichText::new(text).size(12.)))
                .gap(6.)
                .frame(false),
        )
        .on_hover_text(format!("Use only {name}."));
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), name));
    response
}
