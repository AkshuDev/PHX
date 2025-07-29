use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    // Get the target architecture and OS
    let target = env::var("CARGO_CFG_TARGET").unwrap_or_else(|_| "unknown-target".to_string());
    let version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "unknown-version".to_string());

    // Split the target into its components (arch, os)
    let parts: Vec<&str> = target.split('-').collect();
    let arch = parts.get(0).unwrap_or(&"unknown");
    let os = parts.get(1).unwrap_or(&"unknown");

    // Define the output directory structure
    let output_dir = format!("platform/{}/{}/", arch, os);

    // Inform Cargo to rerun the build script if anything changes
    println!("cargo:rerun-if-changed=build.rs");

    // Set the custom output directory
    println!("cargo:target-dir={}", output_dir);

    // Set the custom binary name (phx-<arch>-<os>-<version>)
    let bin_name = format!("phx-{}-{}-{}", arch, os, version);
    println!("cargo:bin-name={}", bin_name);
}
