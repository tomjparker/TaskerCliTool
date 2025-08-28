use crate::config::Config;
use crate::errors::TaskerError;
use std::collections::HashSet;
use std::process::Command;

pub fn run_task(name: &str, config: &Config) -> Result<(), TaskerError> {
    let mut visited = HashSet::new();
    run_recursive(name, config, &mut visited)
}

fn shell_status(cmd: &str) -> Result<std::process::ExitStatus, std::io::Error> {
        if cfg!(target_os = "windows") {
            std::process::Command::new("cmd").arg("/C").arg(cmd).status()
        } else {
            std::process::Command::new("sh").arg("-c").arg(cmd).status()
        }
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
        let mut deps_sorted = deps.clone(); // copy the Vec<String>
        deps_sorted.sort(); // sort alphabetically
        for dep in deps_sorted {
            run_recursive(&dep, config, visited)?;
        }
    }

    println!("Running task: {} -> {}", name, task.cmd);

    let status = shell_status(&task.cmd)?;
    if !status.success() {
        return Err(TaskerError::CommandFailed { task: name.to_string(), code: status.code() });
    }

    Ok(())
}