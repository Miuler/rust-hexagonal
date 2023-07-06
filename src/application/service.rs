use derive_builder::Builder;

use crate::domain::AiSettings;
use crate::domain::error::LlmError;
use crate::domain::llm_repository::{LlmRepository, LlmRepositoryImpl};
use crate::domain::store_repository::{StoreRepository, StoreRepositoryImpl};

#[derive(Default, Clone, Builder)]
pub struct LlmService {
    llm_repository: LlmRepository,
    store_repository: StoreRepository,
}

impl LlmService {
    pub async fn llm_execute(&self, ai_settings: AiSettings) -> Result<String, LlmError> {
        let workspace_path = self.store_repository.create_workspace()?;
        let ai_settings_path = format!("{}/ai_settings.yaml", workspace_path);
        self.store_repository.save_file_in_workspace(&ai_settings_path, ai_settings.into())?;

        let result = self.llm_repository.llm_execute(ai_settings_path.clone(), workspace_path).await;
        // self.store_repository.destroy_workspace();

        result
    }
}

#[cfg(test)]
mod test {
    use crate::domain::AiSettings;

    use super::*;

    #[tokio::test]
    async fn test() {
        let mut ai_settings = AiSettings::default();
        ai_settings.ai_name = "test".to_string();
        ai_settings.ai_role = "test".to_string();

        let llm_service = LlmService::default();
        let result = llm_service.llm_execute(ai_settings).await;
        print!("result:    {:?}\n\n", result);

        assert!(result.is_ok());
    }
}
