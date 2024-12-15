use super::{daily_mission::DailyMission, daily_mission_id::DailyMissionId, user_id::UserId};

#[derive(Debug, Clone)]
pub struct DailyMissionBuilder {
    user_id: UserId,
    mission_id: DailyMissionId,
    title: String,
    description: Option<String>,
}

impl DailyMissionBuilder {
    fn new() -> Self {
        DailyMissionBuilder::default()
    }

    fn set_user_id(mut self, user_id: UserId) -> DailyMissionBuilder {
        self.user_id = user_id;
        self
    }

    fn set_mission_id(mut self, mission_id: DailyMissionId) -> DailyMissionBuilder {
        self.mission_id = mission_id;
        self
    }

    fn set_title(mut self, title: &str) -> DailyMissionBuilder {
        self.title = title.to_string();
        self
    }

    fn set_description(mut self, description: Option<String>) -> DailyMissionBuilder {
        self.description = description;
        self
    }

    fn build(self) -> DailyMission {
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
