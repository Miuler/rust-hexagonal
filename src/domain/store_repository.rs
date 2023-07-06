use derive_builder::Builder;

use crate::domain::error::LlmError;

#[derive(Clone, Builder)]
pub struct StoreRepository {
    pub(crate) workspace_base_path: String,
}

pub trait StoreRepositoryImpl {
    fn save_file_in_workspace(&self, ai_settings_path: &String, ai_settings: String) -> Result<(), LlmError>;
    fn create_workspace(&self) -> Result<String, LlmError>;
    fn destroy_workspace(&self);
}