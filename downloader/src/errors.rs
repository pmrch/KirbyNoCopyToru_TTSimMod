use thiserror::Error;

#[derive(Error, Debug)]
pub enum KirbyError {
    #[error("Url parse error occured: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("JSON (de)serialization error occured: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Standard I/O error occured: {0}")]
    Io(#[from] std::io::Error),

    #[error("Reqwest error occured: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("A semaphore acqusiation error occured: {0}")]
    SyncSemphore(#[from] tokio::sync::AcquireError),

    #[error("A tokio task join error occured: {0}")]
    TaskJoin(#[from] tokio::task::JoinError),

    #[error("Custom error occured: {0}")]
    CustomError(#[from] anyhow::Error),
}

impl From<&str> for KirbyError {
    fn from(err: &str) -> Self { Self::CustomError(anyhow::anyhow!("{err}")) }
}
