//! Implementacion de los diferentes From para convertir de dominio a infraestructura

use crate::domain::AiSettings;

impl From<String> for AiSettings {
    fn from(ai_settings: String) -> Self {
        serde_yaml::from_str(ai_settings.as_str()).unwrap()
    }
}

impl From<AiSettings> for String {
    fn from(ai_settings: AiSettings) -> Self {
        serde_yaml::to_string(&ai_settings).unwrap()
    }
}
