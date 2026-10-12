use super::Result;
use sha2::{Digest, Sha256};
use std::{fs, path::Path};

pub(super) fn checked_read(root: &Path, file: &str, expected: &str) -> Result<String> {
    let path = root.join("data/raw").join(file);
    let bytes = fs::read(&path)?;
    let actual = format!("{:x}", Sha256::digest(&bytes));
    if actual != expected {
        return Err(format!(
            "{file}: checksum mismatch; review the source update and catalog hash"
        )
        .into());
    }
    Ok(String::from_utf8(bytes)?)
}
