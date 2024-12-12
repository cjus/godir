#[allow(unused_imports)]
use std::fs::{self, File};
use std::io;
use std::path::Path;

use clap::Parser;
use regex::Regex;
use serde::{Deserialize, Serialize};
use dirs::home_dir;

/// Command-line program for directory navigation based on patterns.
#[derive(Parser)]
struct Cli {
    /// The pattern to match a directory
    pattern: String,
}

#[derive(Serialize, Deserialize)]
struct Config {
    directories: Vec<String>,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();

    // Locate .godir directory
    let godir_root = home_dir()
        .expect("Could not determine home directory")
        .join(".godir");
    let config_path = godir_root.join("directories.json");

    // Load or initialize configuration
    let config = load_or_initialize_config(&config_path)?;

    // Match directories based on pattern
    let matches = find_matches(&config.directories, &args.pattern)?;

    if matches.is_empty() {
        eprintln!("No matching directories found for pattern: {}", args.pattern);
    } else if matches.len() == 1 {
        // If exactly one match, output just the path
        println!("{}", matches[0]);
    } else {
        // Display a list of matches for user selection
        eprintln!("Multiple matches found:");
        for (i, dir) in matches.iter().enumerate() {
            eprintln!("{}: {}", i + 1, dir);
        }
        eprintln!("Enter the number of the directory to navigate to:");

        // Get user selection
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let choice: usize = input.trim().parse().unwrap_or(0);

        if choice > 0 && choice <= matches.len() {
            println!("{}", matches[choice - 1]);
        } else {
            eprintln!("Invalid choice.");
        }
    }

    Ok(())
}

fn load_or_initialize_config(config_path: &Path) -> io::Result<Config> {
    if !config_path.exists() {
        // Create .godir directory and default config
        fs::create_dir_all(config_path.parent().unwrap())?;
        let default_config = Config { directories: vec![] };
        save_config(config_path, &default_config)?;
    }

    let file = File::open(config_path)?;
    let config: Config = serde_json::from_reader(file)?;
    Ok(config)
}

fn save_config(config_path: &Path, config: &Config) -> io::Result<()> {
    let file = File::create(config_path)?;
    serde_json::to_writer_pretty(file, config)?;
    Ok(())
}

fn find_matches(directories: &[String], pattern: &str) -> io::Result<Vec<String>> {
    let regex = Regex::new(pattern).map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid regex pattern"))?;
    Ok(directories
        .iter()
        .filter(|dir| regex.is_match(dir))
        .cloned()
        .collect())
}