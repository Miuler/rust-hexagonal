//! Esta es la implementacion para manipular el workspace en modo local,
//! por lo que va contra el sistema de archivos local.

use std::fs;

use chrono::Utc;
use rand::distributions::Alphanumeric;
use rand::Rng;

use crate::domain::error::LlmError;
use crate::domain::store_repository::{StoreRepository, StoreRepositoryImpl};

const WORKSPACE: &str = "autogpt_";

impl Default for StoreRepository {
    fn default() -> Self {
        Self {
            workspace_base_path: "/tmp".to_string(),
        }
    }
}

impl StoreRepositoryImpl for StoreRepository {
    fn save_file_in_workspace(&self, ai_settings_path: &String, ai_settings: String) -> Result<(), LlmError> {
        // save string to file
        Ok(fs::write(ai_settings_path, ai_settings)?)
    }

    fn create_workspace(&self) -> Result<String, LlmError> {
        let random_string: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(4)
            .map(char::from)
            .collect();


        let now = Utc::now().format("%Y_%m_%d_%H_%M_%S").to_string();
        let path = format!("{}/{}{}_{}", self.workspace_base_path, WORKSPACE, now, random_string);
        fs::create_dir_all(&path)?;
        Ok(path)
    }

    fn destroy_workspace(&self) {
        todo!()
    }
}
