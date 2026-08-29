use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Deserialize)]
pub struct PackageFile {
    pub info: PackageInfo,
    pub build: Build,
}

#[derive(Deserialize)]
pub struct PackageInfo {
    pub name: String,
    pub description: String,
    pub version: String,
    pub source: String,
}

#[derive(Deserialize)]
pub struct Build {
    pub inst: String,
}

pub fn read_package_info<P: AsRef<Path>>(
    path: P,
) -> Result<PackageFile, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let info: PackageFile = toml::from_str(&content)?;

    Ok(info)
}
