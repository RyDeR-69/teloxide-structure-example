#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Dialogue error: {0}")]
    Dialogue(#[from] teloxide::dispatching::dialogue::InMemStorageError),
    #[error("Request error: {0}")]
    Request(#[from] teloxide::RequestError),
    #[error("Download error: {0}")]
    Download(#[from] teloxide::DownloadError),
}
