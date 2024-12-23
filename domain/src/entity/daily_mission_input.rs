use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct DailyMissionInput {
    pub title: String,
    pub description: Option<String>,
}
