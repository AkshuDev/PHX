mod config;
mod config_structs;

pub use config::read_config;
pub use config_structs::Config;
pub use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use indicatif::{ProgressBar, ProgressStyle};
use std::env;
use std::path::PathBuf;

pub fn expand_tilde(path: &str) -> PathBuf {
    if let Some(stripped) = path.strip_prefix("~/") {
        if let Ok(home) = env::var("HOME") {
            return PathBuf::from(home).join(stripped);
        }
    }
    PathBuf::from(path)
}

pub mod download;

use download::{download_package, package_exists, install_repo};

pub async fn generate_def_config(cfg_path: &str, cfg_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let cfg_path = expand_tilde(cfg_path);
    let cfg_dir = expand_tilde(cfg_dir);
    let pb = ProgressBar::new(100);
    let pb_style = ProgressStyle::default_bar()
            .template("{msg} <{bar:40.yellow/black}> {percent}% | ETA {eta}")
            .unwrap()
            .progress_chars("->=");
    pb.set_style(pb_style);
    pb.set_message(format!("Generating {:?}", cfg_path));
    tokio::fs::create_dir_all(&cfg_dir).await?; // Ensure config folder exists
    pb.inc(25 as u64);
    let mut cfg = File::create(&cfg_path).await?;
    let buffer = b"cache_dir = \"~/.phx/.cache\"\ndebug = false\n";
    pb.inc(50 as u64);
    cfg.write_all(buffer).await?;
    cfg.flush().await?;
    pb.inc(25 as u64);
    pb.finish_with_message(format!("Generated Config: {:?}", cfg_path));
    Ok(())
}

pub async fn install_repository(url: &str, pkg_name: &str, cfg_dir: &str) {
    if package_exists(pkg_name, cfg_dir) {
        println!("[PHX] Package '{}' already exists in cache.", pkg_name);
        return;
    }
    
    match install_repo(url, pkg_name, cfg_dir).await {
        Ok(_) => println!("[PHX] Successfully installed '{}'", pkg_name),
        Err(e) => eprintln!("[PHX] Failed to install '{}': {}", pkg_name, e),
    }
}

pub async fn install_package(pkg_name: &str, cfg_dir: &str) {
    if package_exists(pkg_name, cfg_dir) {
        println!("[PHX] Package '{}' already exists in cache.", pkg_name);
        return;
    }

    let url = format!("https://ppi.example.com/packages/{}.phxpkg", pkg_name);

    match download_package(pkg_name, &url, cfg_dir).await {
        Ok(_) => println!("[PHX] Successfully installed '{}'", pkg_name),
        Err(e) => eprintln!("[PHX] Failed to install '{}': {}", pkg_name, e),
    }
}
