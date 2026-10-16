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
            let height = FLAG_IDS.lines().count().div_ceil(2) * 24;
            let mut pixels = vec![0; 64 * height * 4];
            for (index, cell) in FLAG_PIXELS
                .as_chunks::<{ 32 * 24 * 4 }>()
                .0
                .iter()
                .enumerate()
            {
                for (row, source) in cell.as_chunks::<{ 32 * 4 }>().0.iter().enumerate() {
                    let offset = ((index / 2 * 24 + row) * 64 + index % 2 * 32) * 4;
                    pixels[offset..offset + 32 * 4].copy_from_slice(source);
                }
            }
            let texture = ui.ctx().load_texture(
                "country_flags",
                egui::ColorImage::from_rgba_unmultiplied([64, height], &pixels),
                egui::TextureOptions::LINEAR,
            );
            ui.ctx()
                .data_mut(|data| data.insert_temp(flag_texture_id, texture.clone()));
            texture
        })
}

pub(super) fn cell_uv(index: usize) -> egui::Rect {
    let rows = FLAG_IDS.lines().count().div_ceil(2) as f32;
    egui::Rect::from_min_max(
        egui::pos2((index % 2) as f32 / 2., (index / 2) as f32 / rows),
        egui::pos2((index % 2 + 1) as f32 / 2., (index / 2 + 1) as f32 / rows),
    )
}
