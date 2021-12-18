use thiserror::Error;

#[derive(Debug, Error)]
pub enum InterpreterError {
    #[error("File {0} not found")]
    FileNotFound(String),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}
