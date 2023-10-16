use thiserror::Error;

#[derive(Error, Debug)]
pub enum AxiosError<T> {
    #[error("Request failed: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Deserialization failed: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("Failed to get env: {0}")]
    EnvError(#[from] std::env::VarError),

    #[error("Method error: {0:?}")]
    MethodError(T),
}