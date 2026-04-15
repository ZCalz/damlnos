mod exercises;
mod runner;
mod state;
mod ui;
mod watcher;

use anyhow::{bail, Result};
use clap::{Parser, Subcommand};
use std::process::Command;

#[derive(Parser)]
#[command(
    name = "damlings",
    about = "Learn DAML by fixing broken code",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Watch the current exercise and advance when tests pass (default)
    Watch,
    /// List all exercises and your current progress
    List,
    /// Show a hint for the current exercise
    Hint,
    /// Run a specific exercise by slug (e.g. `damlings run intro1`)
    Run {
        /// Exercise slug, e.g. "intro1", "types3"
        slug: String,
    },
    /// Reset an exercise file back to its original broken state via git
    Reset {
        /// Exercise slug to reset
        slug: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    ui::print_banner();

    match cli.command.unwrap_or(Commands::Watch) {
        Commands::Watch => watcher::run(),
        Commands::List  => cmd_list(),
        Commands::Hint  => cmd_hint(),
        Commands::Run { slug } => cmd_run(&slug),
        Commands::Reset { slug } => cmd_reset(&slug),
    }
}

// ── subcommand handlers ──────────────────────────────────────────────────────

fn cmd_list() -> Result<()> {
    let st = state::State::load()?;
    let all: Vec<&exercises::Exercise> = exercises::EXERCISES.iter().collect();
    ui::print_exercise_list(&all, &st);
    Ok(())
}

fn cmd_hint() -> Result<()> {
    let st = state::State::load()?;
    let ex = &exercises::EXERCISES[st.current];
    ui::print_hint(ex);
    Ok(())
}

fn cmd_run(slug: &str) -> Result<()> {
    let ex = match exercises::find(slug) {
        Some(e) => e,
        None => bail!("No exercise with slug '{slug}'. Run `damlings list` to see all slugs."),
    };

    println!("Running: {}", ex.name);
    println!("File:    {}", ex.file);
    println!();

    let result = runner::run(ex.file)?;
    ui::print_result(&result);
    Ok(())
}

fn cmd_reset(slug: &str) -> Result<()> {
    let ex = match exercises::find(slug) {
        Some(e) => e,
        None => bail!("No exercise with slug '{slug}'. Run `damlings list` to see all slugs."),
    };

    println!("Resetting '{}' to its original state…", ex.name);

    let status = Command::new("git")
        .args(["checkout", "--", ex.file])
        .status()?;

    if !status.success() {
        bail!(
            "git checkout failed. Make sure you are in a git repo and \
             the exercise file has been committed."
        );
    }

    println!("Done. The file has been restored:");
    println!("  {}", ex.file);
    Ok(())
}
