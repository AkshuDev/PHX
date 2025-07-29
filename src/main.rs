use std::env;

mod phx_core;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    
    if args.is_empty() {
        print_help();
        return;
    }

    // Config file parsing first
    let config_path = "~/.phx/config.toml";

    // Load the config
    let config: Result<phx_core::Config, String> = phx_core::read_config(config_path);

    // Debug stuff
    match config {
        Ok(config) => {
            println!("Config loaded: {:?}", config);
            println!("Debug Mode: {:?}", config.debug);
        }
        Err(e) => {
            eprintln!("Failed to load config: {}\n\tContinuing Anyways...", e);
        }
    }

    // Parse args
    let parsed_args = parse_args(args);

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
            }
            "upgrade" => {
                println!("Upgrading all packages...");
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
    println!("Usage: PHX [command] [options]");
    println!("Commands:");
    println!("  update       : Update the package index.");
    println!("  fetch        : Fetch fastest speeds.");
    println!("  search <pkg> : Search for a package.");
    println!("  remove <pkg> : Remove a package.");
    println!("  install <pkg>: Install a package.");
    println!("  upgrade      : Upgrade all packages.");
}
