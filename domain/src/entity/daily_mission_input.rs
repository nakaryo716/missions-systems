use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct DailyMissionInput {
    #[validate(length(min = 1, max = 20))]
    pub title: String,
    #[validate(length(max = 100))]
    pub description: Option<String>,
}
