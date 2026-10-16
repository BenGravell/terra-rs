use super::{layout, persistence, theme};
use crate::{
    model::{Dataset, MapRegion, Score, Settings},
    scoring::evaluate,
};
use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, egui};

#[derive(Default, PartialEq)]
pub(super) enum Tab {
    #[default]
    Results,
    About,
}

#[derive(Resource)]
pub struct TerraApp {
    pub(super) data: Dataset,
    pub(super) map: Vec<MapRegion>,
    pub(super) settings: Settings,
    evaluated: Option<Settings>,
    pub(super) scores: Vec<Score>,
    pub(super) selected: Option<usize>,
    pub(super) reference: Option<usize>,
    pub(super) tab: Tab,
    startup_error: Option<String>,
    pub(super) message: Option<String>,
    styled: bool,
    pub(super) settings_revision: u64,
}

impl Default for TerraApp {
    fn default() -> Self {
        let data = Dataset::parse(include_str!("../../assets/countries.json"));
        let map = MapRegion::parse(include_str!("../../assets/map.json"));
        let startup_error = data
            .as_ref()
            .err()
            .cloned()
            .or_else(|| map.as_ref().err().map(|e| format!("Map data: {e}")));
        let (settings, message) = persistence::restore();
        Self {
            data: data.unwrap_or(Dataset {
                schema_version: 1,
                sources: vec![],
                countries: vec![],
            }),
            map: map.unwrap_or_default(),
            settings,
            evaluated: None,
            scores: vec![],
            selected: None,
            reference: None,
            tab: Tab::Results,
            startup_error,
            message,
            styled: false,
            settings_revision: 0,
        }
    }
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Terra · Find your place".into(),
                canvas: Some("#app".into()),
                fit_canvas_to_parent: true,
                resolution: (1440, 960).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin::default())
        .insert_resource(ClearColor(Color::srgb_u8(
            theme::BACKGROUND.r(),
            theme::BACKGROUND.g(),
            theme::BACKGROUND.b(),
        )))
        .init_resource::<TerraApp>()
        .add_systems(Startup, |mut commands: Commands| {
            commands.spawn((Camera2d, Msaa::Off));
        })
        .add_systems(
            EguiPrimaryContextPass,
            |mut contexts: EguiContexts, mut app: ResMut<TerraApp>| -> Result {
                app.show(contexts.ctx_mut()?);
                Ok(())
            },
        )
        .run();
}

impl TerraApp {
    pub fn show(&mut self, ctx: &egui::Context) {
        if !self.styled {
            theme::apply(ctx);
            self.styled = true;
            // egui installs new font families at the start of the next pass.
            ctx.request_repaint();
            return;
        }
        let mut root = layout::root(ctx);
        layout::header(&mut root, &mut self.tab);
        if let Some(error) = &self.startup_error {
            layout::startup_error(&mut root, error);
            persistence::ready();
            return;
        }
        layout::settings(&mut root, self);
        self.recompute();
        layout::results(&mut root, self);
        persistence::ready();
    }

    fn recompute(&mut self) {
        if self.evaluated.as_ref() == Some(&self.settings) {
            return;
        }
        self.evaluated = Some(self.settings.clone());
        match evaluate(&self.data.countries, &self.settings) {
            Ok(scores) => {
                self.scores = scores;
                if !self
                    .scores
                    .iter()
                    .any(|s| s.matches && Some(s.country) == self.selected)
                {
                    self.selected = self.scores.iter().find(|s| s.matches).map(|s| s.country);
                }
                if let Err(error) = persistence::persist(&self.settings) {
                    self.message = Some(error);
                }
            }
            Err(_) => {
                self.scores.clear();
                self.selected = None;
            }
        }
    }
}

#[cfg(test)]
#[path = "app_test.rs"]
mod tests;
