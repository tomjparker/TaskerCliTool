use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use crate::errors::TaskerError;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub tasks: HashMap<String, TaskConfig>,
}

#[derive(Debug, Deserialize)]
pub struct TaskConfig {
    pub cmd: String,
    pub depends_on: Option<Vec<String>>,
}

pub fn load_config(path: &str) -> Result<Config, TaskerError> {
    let content = fs::read_to_string(path)?;
    let config: Config = serde_yaml::from_str(&content)?;
    Ok(config)
}
