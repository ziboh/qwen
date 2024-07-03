use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("An error occurred when processing a request: {0}")]
    ClientError(#[from] reqwest::Error),
    #[error("Invalid configuration provided: {0}")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),
    #[error("An error (code: {code}) occurred on the API backend: {message}")]
    BackendError {
        /// Message, describing this error
        message: String,
        /// The type of error
        code: String,
    },
    #[error("Invalid env value: {0}")]
    InvalidEnvValue(#[from] std::env::VarError),
}
