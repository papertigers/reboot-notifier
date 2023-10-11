use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Deserialize)]
pub struct Config {
    pub user_key: String,
    pub application_token: String,
    pub message: String,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let mut f = File::open(path).context("failed to open {path}")?;
        let mut buf = String::new();
        f.read_to_string(&mut buf)?;
        let config: Self =
            toml::from_str(&buf).context("failed to parse config file")?;

        Ok(config)
    }
}
