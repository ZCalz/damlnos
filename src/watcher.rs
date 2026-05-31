use crate::{exercises, runner, state::State, ui};
use anyhow::Result;
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc;
use std::time::{Duration, Instant};

/// Debounce window: ignore rapid consecutive saves within this duration.
const DEBOUNCE: Duration = Duration::from_millis(300);

/// Main watch loop. Runs the current exercise, then watches for changes.
pub fn run() -> Result<()> {
    let total = exercises::EXERCISES.len();

    loop {
        let mut st = State::load()?;

        if st.current >= total {
            ui::print_congratulations();
            break;
        }

        let ex = &exercises::EXERCISES[st.current];
        ui::clear_screen();
        ui::print_exercise_banner(st.current, total, ex);

        // Run the exercise once immediately on entry.
        let first = runner::run(ex.file)?;
        ui::print_result(&first);

        if first.passed {
            st.complete(st.current, total);
            st.save()?;
            ui::print_advance(ex, st.current.saturating_sub(1));
            // Loop back to print the next exercise banner.
            continue;
        }

        // Watch the exercise file for saves.
        let (tx, rx) = mpsc::channel::<notify::Result<Event>>();
        let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
        watcher.watch(Path::new(ex.file), RecursiveMode::NonRecursive)?;

        let mut last_run = Instant::now();

        for event in &rx {
            match event {
                Ok(ev) if is_modify(&ev) => {
                    // Debounce: skip if we ran very recently.
                    if last_run.elapsed() < DEBOUNCE {
                        continue;
                    }
                    last_run = Instant::now();

                    // Re-load state in case the user ran `damlnos reset` externally.
                    let mut st = State::load()?;
                    let ex = &exercises::EXERCISES[st.current];

                    ui::clear_screen();
                    ui::print_exercise_banner(st.current, total, ex);
                    let result = runner::run(ex.file)?;
                    ui::print_result(&result);

                    if result.passed {
                        st.complete(st.current, total);
                        st.save()?;
                        ui::print_advance(ex, st.current.saturating_sub(1));
                        // Break inner loop to restart the outer loop for the next exercise.
                        break;
                    }
                }
                Ok(_) => {} // other events (create, access, etc.) — ignore
                Err(e) => eprintln!("Watcher error: {e}"),
            }
        }
    }

    Ok(())
}

fn is_modify(ev: &Event) -> bool {
    matches!(ev.kind, EventKind::Modify(_) | EventKind::Create(_))
}
