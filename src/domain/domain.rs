use derive_builder::Builder;
use display_json::{DebugAsJsonPretty, DisplayAsJsonPretty};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, DisplayAsJsonPretty, DebugAsJsonPretty, PartialEq, Builder)]
pub struct AiSettings {
    pub ai_name: String,
    pub ai_role: String,
    pub ai_goals: Vec<String>,
    pub api_budget: Decimal,
}