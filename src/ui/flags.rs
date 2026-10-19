use bevy_egui::egui;
use std::sync::LazyLock;

pub(super) const FLAG_IDS: &str = include_str!("../../assets/flags/countries.txt");
pub(super) static FLAG_PIXELS: LazyLock<Vec<u8>> = LazyLock::new(|| {
    const PIXELS: &[u32] = include!("../../assets/flags/atlas.rs");
    PIXELS
        .iter()
        .flat_map(|pixel| pixel.to_be_bytes())
        .collect()
});

pub(super) fn texture(ui: &egui::Ui) -> egui::TextureHandle {
    let flag_texture_id = egui::Id::new("country_flags");
    ui.ctx()
        .data_mut(|data| data.get_temp::<egui::TextureHandle>(flag_texture_id))
        .unwrap_or_else(|| {
            let height = FLAG_IDS.lines().count().div_ceil(4) * 48;
            let mut pixels = vec![0; 256 * height * 4];
            for (index, cell) in FLAG_PIXELS
                .as_chunks::<{ 64 * 48 * 4 }>()
                .0
                .iter()
                .enumerate()
            {
                for (row, source) in cell.as_chunks::<{ 64 * 4 }>().0.iter().enumerate() {
                    let offset = ((index / 4 * 48 + row) * 256 + index % 4 * 64) * 4;
                    pixels[offset..offset + 64 * 4].copy_from_slice(source);
                }
            }
            let texture = ui.ctx().load_texture(
                "country_flags",
                egui::ColorImage::from_rgba_unmultiplied([256, height], &pixels),
                egui::TextureOptions::LINEAR,
            );
            ui.ctx()
                .data_mut(|data| data.insert_temp(flag_texture_id, texture.clone()));
            texture
        })
}

pub(super) fn cell_uv(index: usize) -> egui::Rect {
    let rows = FLAG_IDS.lines().count().div_ceil(4) as f32;
    egui::Rect::from_min_max(
        egui::pos2((index % 4) as f32 / 4., (index / 4) as f32 / rows),
        egui::pos2((index % 4 + 1) as f32 / 4., (index / 4 + 1) as f32 / rows),
    )
}
