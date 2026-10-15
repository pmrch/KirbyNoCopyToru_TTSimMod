mod errors;
mod models;
mod prelude;

use std::time::Duration;

use prelude::concurrency::{Arc, JoinHandle, Semaphore, SemaphorePermit};
use prelude::fs::{BufWriter, File, OpenOptions, Path, PathBuf};
use prelude::http::{Client, ClientBuilder, Policy, Response, StreamExt, Url};
use tokio::io::AsyncWriteExt;

use crate::errors::KirbyError;
use crate::models::Card;

type Result<T> = std::result::Result<T, KirbyError>;

async fn download_file<S: AsRef<str>>(client: Arc<Client>, url_str: S, filename: S) -> Result<()> {
    let url: Url = Url::parse(url_str.as_ref())?;
    let filename: &str = filename.as_ref();

    let target: &Path = Path::new("dl_dir");
    if !tokio::fs::try_exists(target).await? {
        tokio::fs::create_dir_all(target).await?;
    }

    let file_path: PathBuf = target.join(filename);
    let file: File = OpenOptions::new().create(true).write(true).append(false).mode(0o755).open(file_path).await?;
    let mut writer: BufWriter<File> = BufWriter::new(file);

    let resp: Response = client.get(url).send().await?;
    if !resp.status().is_success() {
        return Err("Returned non-success code for request".into());
    }

    let mut bytes_stream = resp.bytes_stream();
    while let Some(read) = bytes_stream.next().await {
        let chunk = read?;
        writer.write_all(&chunk).await?;
    }

    writer.flush().await?;
    drop(writer);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let data_str: String = std::fs::read_to_string("sample_json.json")?;
    let json_val: serde_json::Value = serde_json::from_str(&data_str)?;

    let card_vals: &Vec<serde_json::Value> = json_val["cards"].as_array().ok_or("Failed to convert JSON value to array")?;
    let cards_iter = card_vals.iter().cloned().filter_map(|v| serde_json::from_value::<Card>(v).ok());

    let mut download_handles: Vec<JoinHandle<Result<()>>> = Vec::new();
    let client: Client = ClientBuilder::new().connect_timeout(Duration::from_secs(10)).redirect(Policy::limited(3)).build()?;
    let shared_client: Arc<Client> = Arc::new(client);

    let num_cores: usize = num_cpus::get();
    let semaphores: Arc<Semaphore> = Arc::new(Semaphore::new(num_cores));
    for card in cards_iter {
        let sem: Arc<Semaphore> = Arc::clone(&semaphores);
        let client_ref: Arc<Client> = Arc::clone(&shared_client);

        let handle: JoinHandle<Result<()>> = tokio::task::spawn(async move {
            let _permit: SemaphorePermit = sem.acquire().await?;
            download_file(client_ref, card.image.url, card.image.file).await?;
            Ok(())
        });

        download_handles.push(handle);
    }

    for handle in download_handles {
        let res: Result<()> = handle.await?;
        if let Err(e) = res {
            eprintln!("Failed to download this: {e}");
        }
    }

    Ok(())
}
