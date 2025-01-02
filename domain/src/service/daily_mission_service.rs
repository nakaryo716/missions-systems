use sqlx::{MySql, Transaction};

use crate::{
    entity::{
        daily_mission::DailyMission, daily_mission_builder::DailyMissionBuilder,
        daily_mission_id::DailyMissionId, daily_mission_input::DailyMissionInput, token::Token,
    },
    repository::daily_mission_repository::DailyMissionRepository,
};

use super::{
    service_error::daily_mission_service_error::DailyMissionServiceError,
    token_service::TokenService, uuid_service::UUIDService,
};

#[derive(Debug, Clone)]
pub struct DailyMissionService<T, U, M>
where
    T: TokenService,
    U: UUIDService,
    M: DailyMissionRepository,
{
    token_service: T,
    uuid_service: U,
    mission_repo: M,
}

impl<T, U, M> DailyMissionService<T, U, M>
where
    T: TokenService,
    U: UUIDService,
    M: DailyMissionRepository,
{
    pub fn new(token_service: T, uuid_service: U, mission_repo: M) -> Self {
        Self {
            token_service,
            uuid_service,
            mission_repo,
        }
    }

    pub async fn create(
        &self,
        token: Token,
        mission_payload: DailyMissionInput,
    ) -> Result<DailyMissionId, DailyMissionServiceError> {
        let user_id = self.token_service.verify(token)?;
        let mission_id = DailyMissionId(self.uuid_service.generate());
        let mission = DailyMissionBuilder::new()
            .user_id(&user_id)
            .mission_id(&mission_id)
            .title(&mission_payload.title)
            .description(&mission_payload.description)
            .build();

        let mission_id = self.mission_repo.create(&mission).await?;
        Ok(mission_id)
    }

    pub async fn find_by_id(
        &self,
        token: Token,
        mission_id: DailyMissionId,
    ) -> Result<DailyMission, DailyMissionServiceError> {
        let user_id = self.token_service.verify(token)?;
        let mission = self.mission_repo.find_by_id(&mission_id, &user_id).await?;
        Ok(mission)
    }

    pub async fn find_all(
        &self,
        token: Token,
    ) -> Result<Vec<DailyMission>, DailyMissionServiceError> {
        let user_id = self.token_service.verify(token)?;
        let missions = self.mission_repo.find_by_user_id(&user_id).await?;
        Ok(missions)
    }

    pub async fn update(
        &self,
        token: Token,
        mission_id: DailyMissionId,
        mission_payload: DailyMissionInput,
    ) -> Result<(), DailyMissionServiceError> {
        let user_id = self.token_service.verify(token)?;
        let mission = DailyMissionBuilder::new()
            .user_id(&user_id)
            .mission_id(&mission_id)
            .title(&mission_payload.title)
            .description(&mission_payload.description)
            .build();

        self.mission_repo.update(&mission, &user_id).await?;
        Ok(())
    }

    pub async fn set_complete_true<'a>(
        &self,
        mut tx: &'a mut Transaction<'_, MySql>,
        token: Token,
        mission_id: DailyMissionId,
    ) -> Result<(), DailyMissionServiceError> {
        let user_id = self.token_service.verify(token)?;
        self.mission_repo.set_complete_true(&mut tx, &mission_id, &user_id).await?;
        Ok(())
    }

    pub async fn delete(
        &self,
        token: Token,
        mission_id: DailyMissionId,
    ) -> Result<(), DailyMissionServiceError> {
        let user_id = self.token_service.verify(token)?;
        self.mission_repo.delete(&mission_id, &user_id).await?;
        Ok(())
    }
}
