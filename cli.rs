use clap::{Arg, Command};
use crate::runner::run_task;
use crate::config::load_config;

pub fn parse_args() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("tasker")
        .version("0.1.0")
        .author("You")
        .about("Run and manage project tasks")
        .arg(Arg::new("command").required(true))
        .arg(Arg::new("task_name").required(false))
        .get_matches();

    let command = matches.get_one::<String>("command").unwrap().as_str();
    let task_name = matches.get_one::<String>("task_name").map(|s| s.as_str());

    match command {
        "run" => {
            let config = load_config("tasks.yaml")?;
            if let Some(task) = task_name {
                run_task(task, &config)?;
            } else {
                println!("Please specify a task name.");
            }
        }
        "list" => {
            let config = load_config("tasks.yaml")?;
            for name in config.tasks.keys() {
                println!("{}", name);
            }
        }
        _ => println!("Unknown command: {}", command),
    }

    Ok(())
}