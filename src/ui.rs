use crate::{exercises::Exercise, runner::TestResult};
use colored::Colorize;

const DAMLINGS_BANNER: &str = r#"
  ____                    _ _
 |  _ \  __ _ _ __ ___  | (_) ___
 | | | |/ _` | '_ ` _ \ | | |/ _ \
 | |_| | (_| | | | | | || | |  __/
 |____/ \__,_|_| |_| |_||_|_|\___|

 Learn DAML by fixing broken code.
"#;

pub fn print_banner() {
    println!("{}", DAMLINGS_BANNER.cyan().bold());
}

pub fn print_exercise_banner(idx: usize, total: usize, ex: &Exercise) {
    let progress = format!("[{}/{}]", idx + 1, total);
    println!();
    println!("{} {} — {}", progress.yellow().bold(), ex.section.dimmed(), ex.name.bold());
    println!("{} {}", "File:".dimmed(), ex.file.cyan());
    println!();
    println!("{}", "Watching for changes… (edit the file above, save to test)".dimmed());
    println!();
}

pub fn print_result(result: &TestResult) {
    // Strip ANSI escape codes from daml output for clean reprinting.
    let clean = strip_ansi(&result.output);

    if result.compile_error {
        println!("{}", "Compile error:".red().bold());
        // Print only the relevant lines (skip empty lines at start).
        for line in clean.lines().filter(|l| !l.trim().is_empty()) {
            println!("  {line}");
        }
    } else if result.passed {
        println!("{}", "All tests passed!".green().bold());
        for line in clean.lines().filter(|l| l.contains(": ok")) {
            println!("  {}", line.green());
        }
    } else {
        println!("{}", "Tests failed:".red().bold());
        for line in clean.lines().filter(|l| !l.trim().is_empty()) {
            let colored_line = if line.contains(": ok") {
                line.green().to_string()
            } else if line.contains(": failed") {
                line.red().to_string()
            } else {
                line.to_string()
            };
            println!("  {colored_line}");
        }
    }
    println!();
}

pub fn print_advance(ex: &Exercise, _prev_idx: usize) {
    println!("{} {}", "".green().bold(), format!("'{}' complete!", ex.name).green());
    println!();
}

pub fn print_congratulations() {
    println!();
    println!("{}", "All exercises complete! Congratulations!".green().bold());
    println!();
    println!("You now know the fundamentals of DAML.");
    println!("Next steps:");
    println!("  • Read the DAML docs: https://docs.daml.com");
    println!("  • Explore the DAML Finance library");
    println!("  • Build your own smart contract application");
    println!();
}

pub fn print_hint(ex: &Exercise) {
    println!();
    println!("{} {}", "Hint for".yellow(), ex.name.bold());
    println!();
    println!("  {}", ex.hint);
    println!();
}

pub fn print_exercise_list(exercises: &[&crate::exercises::Exercise], state: &crate::state::State) {
    println!();
    println!("{}", "Exercise list:".bold());
    println!();

    let mut current_section = "";
    for (i, ex) in exercises.iter().enumerate() {
        if ex.section != current_section {
            println!("  {}", ex.section.bold().underline());
            current_section = ex.section;
        }
        let status = if state.is_complete(i) {
            "DONE ".green().to_string()
        } else if state.current == i {
            "> ".yellow().bold().to_string()
        } else {
            "  ".to_string()
        };
        println!("  {} {}", status, ex.name);
    }
    println!();
}

/// Remove ANSI escape sequences from a string for clean terminal output.
fn strip_ansi(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            // Skip until 'm'
            for ch in chars.by_ref() {
                if ch == 'm' {
                    break;
                }
            }
        } else {
            out.push(c);
        }
    }
    out
}
