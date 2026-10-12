use super::{Result, map_input::MapInput, raw::checked_read};
use serde_json::Value;
use std::path::Path;
use terra::model::{MapPoint, MapRegion};

pub(super) fn load(root: &Path, input: &MapInput) -> Result<Vec<MapRegion>> {
    let map = map_regions(&checked_read(root, &input.file, &input.sha256)?)?;
    MapRegion::parse(&serde_json::to_string(&map)?)?;
    Ok(map)
}

fn map_regions(text: &str) -> Result<Vec<MapRegion>> {
    let geo: Value = serde_json::from_str(text)?;
    let mut regions = Vec::new();
    for feature in geo["features"].as_array().ok_or("Missing map features")? {
        let props = &feature["properties"];
        let id = ["ISO_A3_EH", "ISO_A3", "ADM0_A3"]
            .iter()
            .filter_map(|k| props[k].as_str())
            .find(|s| *s != "-99")
            .ok_or("Missing map ID")?;
        let id = if id == "KOS" { "XKX" } else { id };
        let name = props["ADMIN"]
            .as_str()
            .ok_or("Missing map name")?
            .to_string();
        let coords = feature["geometry"]["coordinates"]
            .as_array()
            .ok_or("Missing map coordinates")?;
        let polygons = match feature["geometry"]["type"].as_str() {
            Some("Polygon") => vec![&feature["geometry"]["coordinates"]],
            Some("MultiPolygon") => coords.iter().collect(),
            _ => return Err("Unsupported map geometry".into()),
        };
        let mut points = Vec::new();
        let mut indices = Vec::new();
        let mut rings = Vec::new();
        for polygon in polygons {
            let mut vertices = Vec::new();
            let mut holes = Vec::new();
            for (i, ring) in polygon
                .as_array()
                .ok_or("Invalid polygon")?
                .iter()
                .enumerate()
            {
                if i > 0 {
                    holes.push(vertices.len() / 2);
                }
                let ring = ring.as_array().ok_or("Invalid ring")?;
                let start = (points.len() + vertices.len() / 2) as u32;
                rings.push((start..start + ring.len().saturating_sub(1) as u32).collect());
                for point in &ring[..ring.len().saturating_sub(1)] {
                    let lon = point[0].as_f64().ok_or("Invalid longitude")?;
                    let lat = point[1].as_f64().ok_or("Invalid latitude")?;
                    if !(-180.0001..=180.0001).contains(&lon)
                        || !(-90.0001..=90.0001).contains(&lat)
                    {
                        return Err("Map coordinate outside globe".into());
                    }
                    vertices.extend([(lon + 180.) / 360., (90. - lat) / 180.]);
                }
            }
            let base = points.len() as u32;
            let triangles = earcutr::earcut(&vertices, &holes, 2)?;
            indices.extend(triangles.iter().map(|i| base + *i as u32));
            points.extend(vertices.chunks_exact(2).map(|p| MapPoint {
                x: p[0] as f32,
                y: p[1] as f32,
            }));
        }
        regions.push(MapRegion {
            id: id.into(),
            name,
            points,
            indices,
            rings,
        });
    }
    regions.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(regions)
}
