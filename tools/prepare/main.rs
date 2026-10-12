mod catalog;
mod countries;
mod country;
mod coverage;
mod dataset;
mod input;
mod map;
mod map_input;
mod normalization;
mod output;
mod prepare;
mod raw;
mod source;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let check = std::env::args().any(|a| a == "--check");
    prepare::run(root, check)
}
