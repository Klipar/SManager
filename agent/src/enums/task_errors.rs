#[derive(Debug)]
pub enum TaskError {
    FailedToRunTask,
    TaskNotFound,
    TaskAlreadyRunning,
    DatabaseError,
    FailedToPrepareEnvironment(String),
}