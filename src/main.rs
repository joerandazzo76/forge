mod db;

use clap::{Parser, Subcommand};
use std::path::Path;
use std::fs;

use db::*;

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

    let conn = ensure_db().unwrap();
    let project_id = create_project(&conn, name, base.to_str().unwrap()).unwrap();
    create_default_milestones(&conn, project_id).unwrap();

    println!("Project {} created and stored.", name);
}

fn cmd_status() {
    let conn = ensure_db().unwrap();

    match get_latest_project(&conn).unwrap() {
        Some(project) => {
            println!("[forge] Project: {}", project.name);

            if let Some((_, name, _)) = get_next_milestone(&conn, project.id).unwrap() {
                println!("Next milestone: {}", name);
            } else {
                println!("All milestones complete.");
            }
        }
        None => println!("No project found."),
    }
}

fn cmd_run() {
    let conn = ensure_db().unwrap();

    if let Some(project) = get_latest_project(&conn).unwrap() {
        if let Some((id, name, _)) = get_next_milestone(&conn, project.id).unwrap() {
            println!("Running milestone: {}", name);

            add_log(&conn, Some(project.id), "info", &format!("Running milestone: {}", name)).unwrap();

            mark_milestone_done(&conn, id).unwrap();

            println!("✔ milestone completed");
        } else {
            println!("No pending milestones.");
        }
    } else {
        println!("No project found.");
    }
}

fn cmd_next() {
    let conn = ensure_db().unwrap();

    if let Some(project) = get_latest_project(&conn).unwrap() {
        if let Some((_, name, _)) = get_next_milestone(&conn, project.id).unwrap() {
            println!("Next milestone: {}", name);
        } else {
            println!("No pending milestones.");
        }
    }
}

fn cmd_logs() {
    let conn = ensure_db().unwrap();

    let logs = get_recent_logs(&conn, 10).unwrap();
    for (level, message, ts) in logs {
        println!("[{}] {} - {}", level, ts, message);
    }
}

fn cmd_release() {
    println!("[forge] Release (stub)");
}