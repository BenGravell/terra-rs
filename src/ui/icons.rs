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
