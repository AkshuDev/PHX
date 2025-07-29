use serde::{Deserialize};

#[derive(Deserialize, Debug)] // Derive Deserialize to read from toml
pub struct Config {
    pub cache_dir: Option<String>,
    pub debug: Option<bool>
}