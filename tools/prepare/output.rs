use super::Result;
use std::{fs, path::Path};

pub(super) fn output(
    root: &Path,
    name: &str,
    value: &impl serde::Serialize,
    check: bool,
) -> Result<()> {
    let text = if name == "assets/countries.json" {
        serde_json::to_string_pretty(value)?
    } else {
        serde_json::to_string(value)?
    } + "\n";
    let path = root.join(name);
    if check {
        if fs::read_to_string(&path)? != text {
            return Err(format!("{name} is stale; run cargo run -p terra-prepare").into());
        }
    } else {
        fs::write(path, text)?;
    }
    Ok(())
}
