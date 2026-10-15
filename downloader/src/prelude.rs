pub mod concurrency {
    pub use std::sync::Arc;

    pub use tokio::sync::{Semaphore, SemaphorePermit};
    pub use tokio::task::JoinHandle;
}

pub mod fs {
    pub use std::path::{Path, PathBuf};

    pub use tokio::fs::{File, OpenOptions};
    pub use tokio::io::BufWriter;
}

pub mod http {
    pub use futures_util::StreamExt;
    pub use reqwest::redirect::Policy;
    pub use reqwest::{Client, ClientBuilder, Response};
    pub use url::Url;
}
