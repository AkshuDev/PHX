use futures_util::StreamExt;
use reqwest::Client;
use std::path::Path;
use std::time::Duration;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::task;
use indicatif::{ProgressBar, ProgressStyle};
use git2::Repository;

/// Check if the package already exists in cache
pub fn package_exists(pkg_name: &str, cache_dir: &str) -> bool {
    let path = format!("{}/{}.phxpkg", cache_dir, pkg_name);
    Path::new(&path).exists()
}

/// Install a package from a Git repository URL
pub async fn install_repo(repo_url: &str, pkg_name: &str, cache_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Create cache folder if it doesn't exist
    let cache_dir = format!("{}/{}", cache_dir, pkg_name);
    if Path::new(&cache_dir).exists() {
        println!("[PHX] Package '{}' already cloned.", pkg_name);
        return Ok(());
    }

    println!("[PHX] Cloning '{}' into '{}'", repo_url, cache_dir);

    let pb = ProgressBar::new_spinner();
    let pb_style = ProgressStyle::default_bar()
        .template("[PHX:INSTALL] [{bar:40.yellow/black}] {percent}% ION {spinner} ETA:{eta}")
        .unwrap()
        .progress_chars("->="); // The Ion Stream characters
    pb.set_style(pb_style);

    pb.enable_steady_tick(Duration::from_millis(80));
    pb.set_message(format!("Initiating stream for '{}'", pkg_name));

    // Run the git clone in a blocking task (so UI doesn’t freeze)
    let repo_url = repo_url.to_string();
    let cache_dir_clone = cache_dir.clone();
    pb.inc(30 as u64);
    let clone_result = task::spawn_blocking(move || Repository::clone(&repo_url, &cache_dir_clone)).await?;

    // Stop the progress bar
    pb.finish_with_message(format!("Repository '{}' installed successfully.", pkg_name));
    match clone_result {
        Ok(_) => {
            Ok(())
        }
        Err(e) => {
            Err(Box::new(e))
        }
    }
}

/// Download a single package asynchronously
pub async fn download_package(pkg_name: &str, url: &str, cache_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Prepare destination
    let dest_path = format!("{}/{}.phxpkg", cache_dir, pkg_name);
    tokio::fs::create_dir_all(cache_dir).await?; // Ensure cache folder exists

    // Create HTTP client
    let client = Client::new();
    let resp = client.get(url).send().await?;

    // Get content length if available
    let total_size = resp
        .content_length()
        .unwrap_or(0);

    // Setup progress bar
    let pb = ProgressBar::new(total_size);
    let pb_style = ProgressStyle::default_bar()
            .template("{msg} <{bar:40.yellow/black}> {bytes}/{total_bytes} | ETA {eta}")
            .unwrap()
            .progress_chars("->=");
    pb.set_style(pb_style);
    pb.set_message(format!("Downloading {}", pkg_name));

    // Open destination file
    let mut file = File::create(&dest_path).await?;
    let mut stream = resp.bytes_stream();

    // Stream the response into file
    while let Some(chunk) = stream.next().await {
        let data = chunk?;
        file.write_all(&data).await?;
        pb.inc(data.len() as u64);
    }

    pb.finish_with_message(format!("Downloaded {}", pkg_name));

    Ok(())
}
