use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("HTTP error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Env error: {0}")]
    Env(#[from] std::env::VarError),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = anyhow::Result<T, anyhow::Error>;
