use super::{daily_mission::DailyMission, daily_mission_id::DailyMissionId, user_id::UserId};

#[derive(Debug, Clone)]
pub struct DailyMissionBuilder {
    user_id: UserId,
    mission_id: DailyMissionId,
    title: String,
    description: Option<String>,
}

impl DailyMissionBuilder {
    pub fn new() -> Self {
        DailyMissionBuilder::default()
    }

    pub fn user_id(mut self, user_id: &UserId) -> DailyMissionBuilder {
        self.user_id = user_id.to_owned();
        self
    }

    pub fn mission_id(mut self, mission_id: &DailyMissionId) -> DailyMissionBuilder {
        self.mission_id = mission_id.to_owned();
        self
    }

    pub fn title(mut self, title: &str) -> DailyMissionBuilder {
        self.title = title.to_string();
        self
    }

    pub fn description(mut self, description: &Option<String>) -> DailyMissionBuilder {
        self.description = description.to_owned();
        self
    }

    pub fn build(self) -> DailyMission {
        DailyMission {
            user_id: self.user_id,
            mission_id: self.mission_id,
            title: self.title,
            description: self.description,
            is_complete: false,
        }
    }
}

impl Default for DailyMissionBuilder {
    fn default() -> Self {
        Self {
            user_id: UserId(String::default()),
            mission_id: DailyMissionId(String::default()),
            title: String::default(),
            description: Option::default(),
        }
    }
}
