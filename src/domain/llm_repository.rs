use derive_builder::Builder;

use crate::domain::error::LlmError;

#[derive(Default, Clone, Builder)]
pub struct LlmRepository {}

pub trait LlmRepositoryImpl {
    async fn llm_execute(&self, ai_settings_path: String, workspace_directory: String) -> Result<String, LlmError>;
}
