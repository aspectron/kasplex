use thiserror::Error;
use wasm_bindgen::JsValue;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Custom(String),

    #[error("Http error: {0}")]
    HttpError(#[from] workflow_http::error::Error),

    #[error("Invalid JSON: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("API Version `{0}` is not supported")]
    ApiVersionNotSupported(u32),
}

impl Error {
    pub fn custom<T: Into<String>>(msg: T) -> Self {
        Error::Custom(msg.into())
    }
}

impl From<JsValue> for Error {
    fn from(err: JsValue) -> Self {
        Self::Custom(err.as_string().unwrap())
    }
}
