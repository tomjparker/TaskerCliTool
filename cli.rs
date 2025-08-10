use clap::{Arg, Command, builder::PossibleValuesParser};
use crate::runner::run_task;
use crate::config::load_config;

pub fn parse_args() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("tasker")
        .version("0.1.0")
        .author("You")
        .about("Run and manage project tasks")
        .arg(
            Arg::new("command")
                .required(true)
                .value_parser(PossibleValuesParser::new(["run", "list"]))
        )
        .arg(
            Arg::new("task_name")
                .required(false)
                .help("The name of the task to run (only for 'run')"),
        )
        .get_matches();

    let command = matches.get_one::<String>("command").map(|s| s.as_str()).ok_or("missing command")?; // avoid unwrap  - prefer to map instead

    let task_name = matches.get_one::<String>("task_name").map(|s| s.as_str());

    match command {
        "run" => {
            let config = load_config("tasks.yaml")?;
            let Some(task) = task_name else {
                let mut names: Vec<_> = config.tasks.keys().cloned().collect();
                names.sort();
                let list = if names.is_empty() { "(no tasks found)".to_string() } else { names.join(", ") };
                return Err(format!("please specify a task name. available: {}", list).into());
            };
            run_task(task, &config)?;
        }
        "list" => {
            let config = load_config("tasks.yaml")?;
            if config.tasks.is_empty() {
                println!("No tasks found in tasks.yaml");
            } else {
                for name in config.tasks.keys() {
                    println!("{}", name);
                }
            }
        }
        _ => println!("Unknown command: {}", command),
    }

    Ok(())
}