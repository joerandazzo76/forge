use clap::{Parser, Subcommand};
use std::path::Path;
use std::fs;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { name } => cmd_new(name),
        Commands::Status => cmd_status(),
        Commands::Run => cmd_run(),
        Commands::Next => cmd_next(),
        Commands::Logs => cmd_logs(),
        Commands::Release => cmd_release(),
    }
}

#[derive(Parser)]
#[command(name = "forge")]
#[command(about = "AI Dev Orchestrator CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    New { name: String },
    Status,
    Run,
    Next,
    Logs,
    Release,
}

fn cmd_new(name: &str) {
    println!("[forge] Creating new project: {}", name);

    let base = Path::new(name);

    if base.exists() {
        println!("Project already exists.");
        return;
    }

    fs::create_dir_all(base.join(".forge")).unwrap();
    fs::create_dir_all(base.join("src")).unwrap();

    fs::write(
        base.join("Forge.toml"),
        format!(
            "[project]\nname = \"{}\"\nversion = \"0.1.0\"\n",
            name
        ),
    )
    .unwrap();

    fs::write(base.join("src/main.rs"), "fn main() { println!(\"Hello from Forge app!\"); }").unwrap();

    println!("Project {} created.", name);
}

fn cmd_status() {
    println!("[forge] Status");
    println!("(placeholder) No state engine yet");
}

fn cmd_run() {
    println!("[forge] Running next milestone...");
    println!("(placeholder) Executing dummy milestone...");

    println!("✔ milestone completed");
    println!("✔ validation passed");
    println!("✔ (future) commit created");
}

fn cmd_next() {
    println!("[forge] Next milestone preview");
    println!("(placeholder) Initialize project workspace");
}

fn cmd_logs() {
    println!("[forge] Logs");
    println!("(placeholder) No logs yet");
}

fn cmd_release() {
    println!("[forge] Release");
    println!("(placeholder) Release system not implemented yet");
}
