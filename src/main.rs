#[allow(unused_imports)]
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::env;

use clap::Parser;
use regex::Regex;
use serde::{Deserialize, Serialize};
use dirs::home_dir;

#[derive(Parser)]
#[command(
    author = "Carlos Justiniano <cjus@ieee.org>",
    version = "0.1.0",
    about = "A fuzzy directory navigation tool",
    long_about = "
Godir (https://github.com/cjus/godir) allows quick navigation to directories 
using pattern matching. It maintains a list of known directories and exclusions
in ~/.godir/directories.json and provides features like:

  * Pattern-based directory matching
  * Full directory scanning
  * Directory exclusion patterns

Examples:
    godir .             # Add current directory
    godir dev           # Match any directory containing 'dev'
    godir ^/Users       # Match directories starting with '/Users'
    godir project$      # Match directories ending with 'project'"
)]
struct Cli {
    /// The pattern to match a directory
    pattern: Option<String>,  // Make pattern optional since --help and --version don't need it
}

#[derive(Serialize, Deserialize)]
struct Config {
    directories: Vec<String>,
    excludes: Vec<String>,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();

    let pattern = match args.pattern {
        Some(p) => p,
        None => {
            eprintln!("Usage: godir <pattern>");
            eprintln!("Try 'godir --help' for more information.");
            return Ok(());
        }
    };

    // Special case: if pattern is "." or looks like a path
    if pattern == "." || pattern.contains('/') || pattern.contains('\\') {
        let path = if pattern == "." {
            env::current_dir()?
        } else {
            expand_path(&pattern)?
        };

        if let Some(dir_str) = path.to_str() {
            if path.is_dir() {
                let godir_root = home_dir()
                    .expect("Could not determine home directory")
                    .join(".godir");
                let config_path = godir_root.join("directories.json");
                
                let mut config = load_or_initialize_config(&config_path)?;
                
                if !config.directories.contains(&dir_str.to_string()) {
                    config.directories.push(dir_str.to_string());
                    config.directories.sort();
                    save_config(&config_path, &config)?;
                    eprintln!("Added directory to config: {}", dir_str);
                }
                println!("{}", dir_str);
                return Ok(());
            } else {
                eprintln!("Not a valid directory: {}", dir_str);
            }
        }
    }

    // Locate .godir directory
    let godir_root = home_dir()
        .expect("Could not determine home directory")
        .join(".godir");
    let config_path = godir_root.join("directories.json");

    // Load or initialize configuration
    let config = load_or_initialize_config(&config_path)?;

    // Match directories based on pattern
    let matches = find_matches(&config.directories, &pattern)?;

    if matches.is_empty() {
        eprintln!("No matching directories found for pattern: {}", pattern);
        eprintln!("Would you like to manually enter the directory path? [y/N]");
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        if input.trim().eq_ignore_ascii_case("y") {
            eprintln!("Enter the full directory path:");
            let mut path_input = String::new();
            io::stdin().read_line(&mut path_input)?;
            let path = path_input.trim();
            
            if Path::new(path).is_dir() {
                let mut config = config;
                config.directories.push(path.to_string());
                config.directories.sort();
                config.directories.dedup();
                save_config(&config_path, &config)?;
                println!("{}", path);
            } else {
                eprintln!("Invalid directory path.");
                eprintln!("Would you like to perform a full directory scan? [y/N]");
                
                let mut scan_input = String::new();
                io::stdin().read_line(&mut scan_input)?;
                
                if scan_input.trim().eq_ignore_ascii_case("y") {
                    eprintln!("Scanning directories...");
                    let mut config = config;
                    let new_matches = scan_directories(&pattern, Path::new("/"), &config.excludes)?;
                    
                    if !new_matches.is_empty() {
                        config.directories.extend(new_matches.iter().cloned());
                        config.directories.sort();
                        config.directories.dedup();
                        save_config(&config_path, &config)?;
                        
                        eprintln!("Found {} new matching directories:", new_matches.len());
                        for dir in &new_matches {
                            eprintln!("  {}", dir);
                        }
                        
                        if new_matches.len() == 1 {
                            println!("{}", new_matches[0]);
                        }
                    } else {
                        eprintln!("No matching directories found after scan.");
                    }
                }
            }
        } else {
            eprintln!("Would you like to perform a full directory scan? [y/N]");
            
            let mut scan_input = String::new();
            io::stdin().read_line(&mut scan_input)?;
            
            if scan_input.trim().eq_ignore_ascii_case("y") {
                eprintln!("Scanning directories...");
                let mut config = config;
                let new_matches = scan_directories(&pattern, Path::new("/"), &config.excludes)?;
                
                if !new_matches.is_empty() {
                    config.directories.extend(new_matches.iter().cloned());
                    config.directories.sort();
                    config.directories.dedup();
                    save_config(&config_path, &config)?;
                    
                    eprintln!("Found {} new matching directories:", new_matches.len());
                    for dir in &new_matches {
                        eprintln!("  {}", dir);
                    }
                    
                    if new_matches.len() == 1 {
                        println!("{}", new_matches[0]);
                    }
                } else {
                    eprintln!("No matching directories found after scan.");
                }
            }
        }
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
        let default_config = Config { 
            directories: vec![],
            excludes: vec![],  // Empty by default, user will populate via directories.json
        };
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

fn scan_directories(pattern: &str, start_path: &Path, excludes: &[String]) -> io::Result<Vec<String>> {
    let regex = Regex::new(pattern)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid regex pattern"))?;
    let mut matches = Vec::new();

    // Basic skip dirs (only those that could cause issues)
    let skip_dirs = vec![
        ".Trash"  // Keep only essential system directories that should always be skipped
    ];

    fn visit_dirs(dir: &Path, regex: &Regex, matches: &mut Vec<String>, skip_dirs: &[&str], excludes: &[String]) -> io::Result<()> {
        // Show current directory being scanned
        if let Some(dir_str) = dir.to_str() {
            // Check if this directory matches any exclusion pattern
            if excludes.iter().any(|excluded| dir_str.contains(excluded)) {
                return Ok(());
            }
            
            eprint!("\rScanning: {}", dir_str);
            io::stderr().flush().ok();
        }

        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                if path.is_dir() {
                    let dir_name = path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("");
                    
                    // Skip hidden and specified directories
                    if dir_name.starts_with('.') || skip_dirs.contains(&dir_name) {
                        continue;
                    }

                    if let Ok(path_str) = path.to_str().ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidInput, "Invalid path")
                    }) {
                        if regex.is_match(path_str) {
                            matches.push(path_str.to_string());
                        }
                    }

                    // Ignore errors when visiting subdirectories
                    let _ = visit_dirs(&path, regex, matches, skip_dirs, excludes);
                }
            }
        }
        Ok(())
    }

    visit_dirs(start_path, &regex, &mut matches, &skip_dirs, excludes)?;
    Ok(matches)
}

fn expand_path(path: &str) -> io::Result<PathBuf> {
    let path_buf = PathBuf::from(path);
    if path_buf.is_absolute() {
        Ok(path_buf)
    } else {
        env::current_dir()?.join(path).canonicalize()
    }
}