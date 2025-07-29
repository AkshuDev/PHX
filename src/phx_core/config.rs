use std::fs::File;
use std::io::Read;
use std::path::Path;
use toml;

use crate::phx_core::config_structs::Config;

// parse the config file
pub fn read_config(config_path: &str) -> Result<Config, String> {
    let path = Path::new(config_path);

    // Open the config file
    let mut file = File::open(path).map_err(|e| format!("Error openenign file: {}", e))?;

    // Read the file contents into a buffer
    let mut config_contents = String::new();
    file.read_to_string(&mut config_contents).map_err(|e| format!("Error reading file: {}", e))?;

    // Parse toml contents into the Config struct
    toml::de::from_str(&config_contents).map_err(|e| format!("Error parsing TOML file: {}", e))
}