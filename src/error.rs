use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("error occurred in translating")]
    TranslateError(#[from] reqwest::Error),

    #[error("decode subtitle error, {0}")]
    SubtitleDecodeError(String),
}
