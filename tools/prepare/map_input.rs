use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct MapInput {
    pub file: String,
    pub sha256: String,
}
