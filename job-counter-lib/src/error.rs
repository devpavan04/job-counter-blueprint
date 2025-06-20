use thiserror::Error;

#[derive(Error, Debug)]
pub enum JobCounterError {
    #[error("Storage error: {0}")]
    Storage(String),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Blueprint SDK error: {0}")]
    Sdk(#[from] blueprint_sdk::Error),
}

pub type Result<T> = std::result::Result<T, JobCounterError>;
