//! App orchestration, responsive layout, and focused egui components.

mod about;
mod app;
mod bars;
mod constraints;
mod culture;
mod flags;
mod header;
mod layout;
mod map;
mod persistence;
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
