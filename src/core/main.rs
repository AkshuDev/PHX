use std::collections::HashMap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    
    if args.is_empty() {
        print_help();
        return;
    }

    let parsed_args = parseArgs(args);

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

fn parseArgs(args: Vec<String>) -> Vec<(String, String)> {
    let mut parsed = Vec::new();
    parsed
}


