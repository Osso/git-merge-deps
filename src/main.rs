mod differ;
mod merger;
mod requirement;

use std::env;
use std::fs;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: git-merge-deps <base> <current> <other>");
        eprintln!();
        eprintln!("Git merge driver for pip requirements files.");
        eprintln!("Merges dependency changes intelligently, always keeping higher versions.");
        return ExitCode::from(1);
    }

    let base_file = &args[1];
    let current_file = &args[2];
    let other_file = &args[3];

    let result = run_merge(base_file, current_file, other_file);

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {e}");
            ExitCode::from(1)
        }
    }
}

fn run_merge(base_file: &str, current_file: &str, other_file: &str) -> Result<(), String> {
    let base_content = fs::read_to_string(base_file).map_err(|e| format!("reading base: {e}"))?;
    let current_content =
        fs::read_to_string(current_file).map_err(|e| format!("reading current: {e}"))?;
    let other_content =
        fs::read_to_string(other_file).map_err(|e| format!("reading other: {e}"))?;

    let base_reqs = requirement::parse_requirements(&base_content);
    let other_reqs = requirement::parse_requirements(&other_content);
    let mut current_reqs = requirement::parse_requirements(&current_content);

    let diff = differ::differ(&base_reqs, &other_reqs);
    merger::merge(&mut current_reqs, diff);

    let output = merger::format_requirements(&current_reqs);
    fs::write(current_file, output).map_err(|e| format!("writing result: {e}"))?;

    Ok(())
}
