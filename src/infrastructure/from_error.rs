//! Implementacion de los diferentes From para convertir de los diferentes errores de infraestructura

use std::io::Error;

use axum::response::{IntoResponse, Response};
use tokio::task::JoinError;

use crate::domain::error::LlmError;

impl IntoResponse for LlmError {
    fn into_response(self) -> Response {
        "Error".into_response()
    }
}

impl From<serde_yaml::Error> for LlmError {
    fn from(value: serde_yaml::Error) -> Self {
        LlmError::GenericError(value.to_string())
    }
}

impl From<std::io::Error> for LlmError {
    fn from(value: Error) -> Self {
        LlmError::GenericError(value.to_string())
    }
}

impl From<JoinError> for LlmError {
    fn from(value: JoinError) -> Self {
        LlmError::GenericError(value.to_string())
    }
}

impl From<pyo3::PyErr> for LlmError {
    fn from(value: pyo3::PyErr) -> Self {
        LlmError::GenericError(value.to_string())
    }
}

