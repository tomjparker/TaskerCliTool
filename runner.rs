use crate::config::Config;
use crate::errors::TaskerError;
use std::collections::HashSet;
use std::process::Command;

pub fn run_task(name: &str, config: &Config) -> Result<(), TaskerError> {
    let mut visited = HashSet::new();
    run_recursive(name, config, &mut visited)
}

fn run_recursive(
    name: &str,
    config: &Config,
    visited: &mut HashSet<String>,
) -> Result<(), TaskerError> {
    if visited.contains(name) {
        return Ok(()); // Prevent circular deps
    }
    visited.insert(name.to_string());

    let task = config.tasks.get(name)
        .ok_or_else(|| TaskerError::TaskNotFound(name.to_string()))?;

    if let Some(deps) = &task.depends_on {
        for dep in deps {
            run_recursive(dep, config, visited)?;
        }
    }

    println!("Running task: {} -> {}", name, task.cmd);
    let status = Command::new("sh")
        .arg("-c")
        .arg(&task.cmd)
        .status()?;

    if !status.success() {
        eprintln!("Task {} failed.", name);
    }

    Ok(())
}