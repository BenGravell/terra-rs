//! App orchestration, responsive layout, and focused egui components.

mod about;
mod app;
mod bars;
mod constraints;
mod continents;
mod culture;
mod disk;
mod flags;
mod header;
mod help;
mod icons;
mod layout;
mod map;
mod persistence;
mod quality_triangle;
mod ranking;
mod reference;
mod results_summary;
mod settings;
mod slider;
mod theme;
mod weights;

pub use app::{TerraApp, run};

#[cfg(test)]
mod test_support;

#[cfg(test)]
mod browser_smoke_test;
