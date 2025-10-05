use std::env;

mod phx_core;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    
    if args.is_empty() {
        print_help();
        return;
    }

    // Config file parsing first
    let config_path = "~/.phx/config.toml";
    let phx_dir = "~/.phx";
    let exp_cfg_path = phx_core::expand_tilde(config_path)
        .to_string_lossy()
        .to_string();
    let exp_phx_dir = phx_core::expand_tilde(phx_dir)
        .to_string_lossy()
        .to_string();

    // Load the config
    let mut config: Result<phx_core::Config, String> = phx_core::read_config(&exp_cfg_path);
    let mut cache_dir = String::from(".cache");

    // Debug stuff
    match config {
        Ok(ref config) => {
            println!("Config loaded: {:?}", config);
            println!("Debug Mode: {:?}", config.debug);
            println!("Cache Dir: {:?}", config.cache_dir);
            if let Some(ref dir) = config.cache_dir {
                cache_dir = phx_core::expand_tilde(dir)
                    .to_string_lossy()
                    .to_string();
            }
        }
        Err(ref e) => {
            eprintln!("Failed to load config: {}\n\tContinuing Anyways...", e);
        }
    }

    // Parse args
    let parsed_args = parse_args(args);
    let mut pkgname = String::from("phx_pkg");

    for (key, value) in parsed_args {
        match key.as_str() {
            "update" => {
                println!("Updating package index...");
            }
            "fetch" => {
                println!("Fetching fastest speeds...");
            }
            "search" => {
                println!("Searching for package - {}", value);
            }
            "remove" => {
                println!("Removing package - {}", value);
            }
            "install" => {
                println!("Installing package - {}", value);
                phx_core::install_package(&value, &cache_dir).await;
            }
            "upgrade" => {
                println!("Upgrading all packages...");
            }
            "pkg" => {
                println!("Package name - {}", value);
                pkgname = value.to_string();
            }
            "genconfig" => {
                println!("Generating config file and all the requried folders...");
                phx_core::generate_def_config(config_path, phx_dir).await;
                config = phx_core::read_config(&exp_cfg_path);
                match config {
                    Ok(ref config) => {
                        println!("Config loaded: {:?}", config);
                        println!("Debug Mode: {:?}", config.debug);
                        println!("Cache Dir: {:?}", config.cache_dir);
                        if let Some(ref dir) = config.cache_dir {
                            cache_dir = phx_core::expand_tilde(dir)
                                .to_string_lossy()
                                .to_string();
                        }
                    }
                    Err(ref e) => {
                        eprintln!("Failed to load config: {}\n\tContinuing Anyways...", e);
                    }
                }
            }
            
            "instrepo" => {
                println!("Installing Package - {}", value);
                phx_core::install_repository(&value, &pkgname,&cache_dir).await;
            }

            _ => {
                println!("Unknown command - {}:{}", key, value);
            }
        }
    }
}

fn parse_args(args: Vec<String>) -> Vec<(String, String)> {
    let mut parsed = Vec::new();
    
    for arg in args {    
        // Remove leading hyphens
        let cleaned_arg = arg.trim_start_matches('-').to_string();

        // Check if the arg contains a seperator
        if let Some((key, value)) = cleaned_arg.split_once(':') {
            parsed.push((key.to_string(), value.to_string()));
        } else if let Some((key, value)) = cleaned_arg.split_once('=') {
            parsed.push((key.to_string(), value.to_string()));
        } else {
            // No seperartor hence flag
            parsed.push((cleaned_arg, "true".to_string()));
        }
    }

    parsed
}

fn print_help() {
    println!("Usage: PHX [commands]");
    println!("Commands:");
    println!("  update         - Update the package index.");
    println!("  fetch          - Fetch fastest speeds.");
    println!("  search:<pkg>   - Search for a package.");
    println!("  remove:<pkg>   - Remove a package.");
    println!("  install:<pkg>  - Install a package.");
    println!("  upgrade        - Upgrade all packages.");
    println!("  instrepo:<url> - Install a Repository.");
    println!("  pkg:<pkg name> - Set the package name.");
    println!("  genconfig      - Auto generate config file (NOTE: This will overwrite orignal config).");
}
