#[derive(Debug)]
pub enum LlmError {
    GenericError(String),
    NotFound { path: String },
}
