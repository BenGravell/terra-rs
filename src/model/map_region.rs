use super::MapPoint;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct MapRegion {
    pub id: String,
    pub name: String,
    /// Equirectangular coordinates normalized to [0, 1].
    pub points: Vec<MapPoint>,
    pub indices: Vec<u32>,
    pub rings: Vec<Vec<u32>>,
}

impl MapRegion {
    pub fn parse(text: &str) -> Result<Vec<Self>, String> {
        let regions: Vec<Self> =
            serde_json::from_str(text).map_err(|e| format!("Map data: {e}"))?;
        if regions.is_empty() {
            return Err("Map data is empty.".into());
        }
        for region in &regions {
            if region.points.is_empty()
                || region.indices.is_empty()
                || !region.indices.len().is_multiple_of(3)
                || region
                    .points
                    .iter()
                    .flat_map(|p| p.values())
                    .any(|x| !x.is_finite() || !(0.0..=1.0).contains(&x))
                || region
                    .indices
                    .iter()
                    .chain(region.rings.iter().flatten())
                    .any(|i| *i as usize >= region.points.len())
            {
                return Err(format!("Invalid map geometry for {}", region.name));
            }
        }
        Ok(regions)
    }

    pub fn contains(&self, p: MapPoint) -> bool {
        let cross = |u: MapPoint, v: MapPoint, w: MapPoint| {
            (u.x - w.x) * (v.y - w.y) - (v.x - w.x) * (u.y - w.y)
        };
        self.indices.as_chunks::<3>().0.iter().any(|t| {
            let [a, b, c] = [
                self.points[t[0] as usize],
                self.points[t[1] as usize],
                self.points[t[2] as usize],
            ];
            if cross(a, b, c).abs() < 1e-12 {
                return false;
            }
            let signs = [cross(p, a, b), cross(p, b, c), cross(p, c, a)];
            signs.iter().all(|s| *s >= 0.) || signs.iter().all(|s| *s <= 0.)
        })
    }
}

#[cfg(test)]
#[path = "map_region_test.rs"]
mod tests;
