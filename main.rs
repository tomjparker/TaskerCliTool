mod cli;
mod config;
mod errors;
mod runner;
mod tasks;

use cli::parse_args;
use runner::run_task;

fn main() {
    if let Err(e) = parse_args() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}