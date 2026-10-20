//! Generate browser inputs from the real layout and verify them without a browser.
use super::{TerraApp, test_support::harness};
use crate::model::Settings;
use bevy_egui::egui;
use egui_kittest::{Harness, kittest::Queryable};
use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "action", rename_all = "snake_case")]
enum Step {
    Click {
        label: &'static str,
        point: [f32; 2],
        hold: bool,
        settings: Settings,
    },
    Reload {
        settings: Settings,
    },
    Resize {
        width: u32,
        height: u32,
    },
    Screenshot {
        name: &'static str,
    },
}

fn click(h: &mut Harness<'_, TerraApp>, steps: &mut Vec<Step>, label: &'static str, slider: bool) {
    let rect = h.get_by_label(label).rect();
    let point = egui::pos2(
        if slider {
            rect.left() + rect.width() * 0.8
        } else {
            rect.center().x
        },
        rect.center().y,
    );
    assert!(h.ctx.content_rect().contains(point), "{label} is offscreen");
    h.hover_at(point);
    h.run();
    h.drag_at(point);
    h.run();
    h.drop_at(point);
    h.run();
    steps.push(Step::Click {
        label,
        point: [point.x, point.y],
        hold: slider,
        settings: h.state().settings.clone(),
    });
}

fn reload(h: &mut Harness<'static, TerraApp>, steps: &mut Vec<Step>) {
    let settings = h.state().settings.clone();
    let size = h.ctx.content_rect().size();
    *h = harness(size.x);
    h.set_size(size);
    h.state_mut().settings = settings.clone();
    h.run();
    steps.push(Step::Reload { settings });
}

fn resize(h: &mut Harness<'_, TerraApp>, steps: &mut Vec<Step>, width: u32, height: u32) {
    h.set_size(egui::vec2(width as f32, height as f32));
    h.run();
    steps.push(Step::Resize { width, height });
}

#[test]
fn browser_smoke_contract() {
    let mut h = harness(1440.);
    let mut steps = vec![];
    resize(&mut h, &mut steps, 1440, 1000);
    let initial = h.state().settings.clone();
    steps.push(Step::Screenshot { name: "desktop" });
    click(&mut h, &mut steps, "Power Distance", true);
    assert!(h.state().settings.culture.power_distance > 50.);
    reload(&mut h, &mut steps);
    click(&mut h, &mut steps, "Reset Culture", false);
    assert_eq!(h.state().settings, initial);
    click(&mut h, &mut steps, "Culture Fit vs Quality of Life", true);
    let weights = &h.state().settings.overall_importances;
    assert!(weights.quality > 5.);
    assert_eq!(weights.culture + weights.quality, 10.);
    assert!(
        [weights.culture, weights.quality]
            .iter()
            .all(|w| (0. ..=10.).contains(w) && w.fract() == 0.)
    );
    reload(&mut h, &mut steps);
    click(&mut h, &mut steps, "Culture", false);
    assert!(h.query_by_label("Power Distance").is_none());
    click(
        &mut h,
        &mut steps,
        "Quality of Life weights triangle",
        false,
    );
    assert!(
        h.state().settings.quality_importances.happy_planet
            > h.state().settings.quality_importances.social_progress
    );
    steps.push(Step::Screenshot { name: "triangle" });
    reload(&mut h, &mut steps);
    click(&mut h, &mut steps, "Culture", false);
    click(&mut h, &mut steps, "Reset Weights", false);
    assert_eq!(h.state().settings, initial);
    resize(&mut h, &mut steps, 390, 844);
    steps.push(Step::Screenshot { name: "narrow" });
    click(&mut h, &mut steps, "Power Distance", true);
    assert!(h.state().settings.culture.power_distance > 50.);
    steps.push(Step::Screenshot {
        name: "narrow-settings",
    });
    click(&mut h, &mut steps, "Weights", false);
    click(&mut h, &mut steps, "Table", false);
    click(
        &mut h,
        &mut steps,
        "Quality of Life weights triangle",
        false,
    );
    steps.push(Step::Screenshot {
        name: "narrow-triangle",
    });
    reload(&mut h, &mut steps);
    if let Some(path) = std::env::var_os("TERRA_SMOKE_PLAN") {
        std::fs::write(path, serde_json::to_vec_pretty(&steps).unwrap()).unwrap();
    }
}
