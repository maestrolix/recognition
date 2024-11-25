#[derive(thiserror::Error, Debug)]
pub enum CreatePhotoError {
    #[error("ORM request error {0}")]
    DieselError(#[from] diesel::result::Error),

    #[error("Tokio StdIO error: {0}")]
    TokioStdIO(#[from] tokio::io::Error),

    #[error("Image error: {0}")]
    ImageError(#[from] image::ImageError),

    #[error("SerdeJson error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("unknown data store error")]
    Unknown,
    // Делал для OPTION
    // #[error("")]
    // Infallible(#[from] std::convert::Infallible),
}
