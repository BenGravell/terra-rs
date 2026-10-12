use super::TerraApp;
use bevy_egui::egui;
use egui_kittest::Harness;

pub(super) fn harness(width: f32) -> Harness<'static, TerraApp> {
    Harness::builder()
        .with_size(egui::vec2(width, 950.))
        .build_ui_state(
            |ui, app: &mut TerraApp| app.show(ui.ctx()),
            TerraApp::default(),
        )
}
