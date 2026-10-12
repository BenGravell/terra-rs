use super::{Overall, Quality};

#[derive(Debug)]
pub struct Score {
    pub country: usize,
    pub rank: usize,
    pub culture: f64,
    pub quality: f64,
    pub overall: f64,
    pub quality_contributions: Quality,
    pub overall_contributions: Overall,
    pub matches: bool,
}
