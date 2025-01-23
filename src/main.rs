use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

// Path to store credentials securely
fn cred_file_path() -> PathBuf {
    let dir = dirs::config_dir().unwrap().join("git-persona");
    if !dir.exists() {
        fs::create_dir_all(&dir).expect("Failed to create config directory");
    }
    dir.join("credentials.json")
}

// Struct for credentials
#[derive(Serialize, Deserialize)]
struct Credential {
    username: String,
    email: String,
}

// Struct for CLI
#[derive(Parser)]
#[command(name = "git-persona")]
#[command(about = "A CLI tool for managing Git credentials", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        name: String,
        username: String,
        email: String,
    },
    List,
    Use {
        name: String,
    },
    Remove {
        name: String,
    },
}

fn load_credentials() -> HashMap<String, Credential> {
    let path = cred_file_path();
    if path.exists() {
        let data = fs::read_to_string(path).expect("Failed to read credentials file");
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        HashMap::new()
    }
}

fn save_credentials(credentials: &HashMap<String, Credential>) {
    let path = cred_file_path();
    let data = serde_json::to_string_pretty(&credentials).expect("Failed to serialize credentials");
    fs::write(path, data).expect("Failed to save credentials");
}

fn add_credential(name: &str, username: &str, email: &str) {
    let mut credentials = load_credentials();
    credentials.insert(name.to_string(), Credential {
        username: username.to_string(),
        email: email.to_string(),
    });
    save_credentials(&credentials);
    println!("Added credentials for {}", name);
}

fn list_credentials() {
    let credentials = load_credentials();
    for (name, cred) in credentials {
        println!("{}: {} <{}>", name, cred.username, cred.email);
    }
}

fn use_credential(name: &str) {
    let credentials = load_credentials();
    if let Some(cred) = credentials.get(name) {
        Command::new("git")
            .args(&["config", "--global", "user.name", &cred.username])
            .output()
            .expect("Failed to set git username");
        Command::new("git")
            .args(&["config", "--global", "user.email", &cred.email])
            .output()
            .expect("Failed to set git email");
        println!("Switched to credentials for {}", name);
    } else {
        println!("No credentials found for {}", name);
    }
}

fn remove_credential(name: &str) {
    let mut credentials = load_credentials();
    if credentials.remove(name).is_some() {
        save_credentials(&credentials);
        println!("Removed credentials for {}", name);
    } else {
        println!("No credentials found for {}", name);
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { name, username, email } => add_credential(&name, &username, &email),
        Commands::List => list_credentials(),
        Commands::Use { name } => use_credential(&name),
        Commands::Remove { name } => remove_credential(&name),
    }
}

