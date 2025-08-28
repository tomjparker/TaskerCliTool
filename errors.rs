use std::fmt;
use std::io;

#[derive(Debug)]
pub enum TaskerError {
    Io(io::Error),
    Parse(serde_yaml::Error),
    TaskNotFound(String),
}

impl fmt::Display for TaskerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskerError::Io(e) => write!(f, "IO error: {}", e),
            TaskerError::Parse(e) => write!(f, "Parse error: {}", e),
            TaskerError::TaskNotFound(name) => write!(f, "Task not found: {}", name),
        }
    }
}

impl std::error::Error for TaskerError {}

impl From<io::Error> for TaskerError {
    fn from(err: io::Error) -> Self {
        TaskerError::Io(err)
    }
}

impl From<serde_yaml::Error> for TaskerError {
    fn from(err: serde_yaml::Error) -> Self {
        TaskerError::Parse(err)
    }
}