//! Domain types use named fields and serialize to JSON arrays in declaration order.

mod country;
mod culture;
mod dataset;
mod location;
mod map_point;
mod map_region;
mod minimums;
mod overall;
mod quality;
mod range;
mod score;
mod settings;
mod source;

pub use country::Country;
pub use culture::Culture;
pub use dataset::Dataset;
pub use location::Location;
pub use map_point::MapPoint;
pub use map_region::MapRegion;
pub use minimums::Minimums;
pub use overall::Overall;
pub use quality::Quality;
pub use range::Range;
pub use score::Score;
pub use settings::Settings;
pub use source::Source;

pub const EPS: f64 = 1e-3;
pub const CONTINENTS: [&str; 6] = [
    "Africa",
    "Asia",
    "Europe",
    "Oceania",
    "North America",
    "South America",
];
