use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub enum TaskError {
    FailedToRunTask,
    TaskNotFound,
    TaskAlreadyRunning,
    DatabaseError,
    FailedToPrepareEnvironment(String),
}

impl fmt::Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskError::FailedToRunTask => write!(f, "FailedToRunTask"),
            TaskError::TaskNotFound => write!(f, "TaskNotFound"),
            TaskError::TaskAlreadyRunning => write!(f, "TaskAlreadyRunning"),
            TaskError::DatabaseError => write!(f, "DatabaseError"),
            TaskError::FailedToPrepareEnvironment(msg) => {
                write!(f, "FailedToPrepareEnvironment: {}", msg)
            }
        }
    }
}