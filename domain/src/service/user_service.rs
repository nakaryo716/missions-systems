use crate::{
    entity::{user::User, user_builder::UserBuilder, user_id::UserId, user_input::UserInput},
    repository::user_repository::UserRepository,
};

use super::{
    password_hash_service::PasswordHashService,
    service_error::user_service_error::UserServiceError, uuid_service::UUIDService,
};

pub struct UserService<P, R, U>
where
    P: PasswordHashService,
    R: UserRepository,
    U: UUIDService,
{
    password_hasher: P,
    user_repo: R,
    uuid_service: U,
}

impl<P, R, U> UserService<P, R, U>
where
    P: PasswordHashService,
    R: UserRepository,
    U: UUIDService,
{
    pub fn new(password_hasher: P, user_repo: R, uuid_service: U) -> Self {
        Self {
            password_hasher,
            user_repo,
            uuid_service,
        }
    }

    pub async fn create_user(&self, user_input: UserInput) -> Result<UserId, UserServiceError> {
        let user_id = UserId(self.uuid_service.generate());
        let password_hash = self
            .password_hasher
            .hash_password(&user_input.password)
            .await?;
        let builder = UserBuilder {
            user_id,
            user_name: user_input.user_name,
            email: user_input.email,
            password_hash,
        };

        let user_id = self.user_repo.create(&builder).await?;
        Ok(user_id)
    }

    pub async fn read_user_by_id(&self, user_id: UserId) -> Result<User, UserServiceError> {
        let user = self.user_repo.find_by_id(&user_id).await?;
        Ok(user)
    }

    pub async fn read_user_by_email(&self, email: String) -> Result<User, UserServiceError> {
        let user = self.user_repo.find_by_email(&email).await?;
        Ok(user)
    }

    pub async fn update_user(&self, user: User) -> Result<(), UserServiceError> {
        self.user_repo.update(&user).await?;
        Ok(())
    }

    pub async fn delete_user(&self, user_id: UserId) -> Result<(), UserServiceError> {
        self.user_repo.delete(&user_id).await?;
        Ok(())
    }

    pub async fn is_user_exist(&self, user_id: UserId) -> Result<bool, UserServiceError> {
        let flag = self.user_repo.is_exist(&user_id).await?;
        Ok(flag)
    }
}
