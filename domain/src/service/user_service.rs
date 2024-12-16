use crate::{
    entity::{
        token::Token, user::User, user_builder::UserBuilder, user_id::UserId, user_input::UserInput,
    },
    repository::user_repository::UserRepository,
};

use super::{
    password_hash_service::PasswordHashService,
    service_error::user_service_error::UserServiceError, token_service::TokenService,
    uuid_service::UUIDService,
};

pub struct UserService<P, T, R, U>
where
    P: PasswordHashService,
    T: TokenService,
    R: UserRepository,
    U: UUIDService,
{
    password_hasher: P,
    token_service: T,
    user_repo: R,
    uuid_service: U,
}

impl<P, T, R, U> UserService<P, T, R, U>
where
    P: PasswordHashService,
    T: TokenService,
    R: UserRepository,
    U: UUIDService,
{
    pub fn new(password_hasher: P, token_service: T, user_repo: R, uuid_service: U) -> Self {
        Self {
            password_hasher,
            token_service,
            user_repo,
            uuid_service,
        }
    }

    pub async fn create_user(&self, user_input: UserInput) -> Result<UserId, UserServiceError> {
        // ユーザーが既に存在するか確認
        // trueの場合は既に存在するため早期リターン
        if self.user_repo.is_exist(&user_input.email).await? {
            return Err(UserServiceError::UserAlreadyExists);
        }

        // ユーザーIDの作成
        let user_id = UserId(self.uuid_service.generate());
        //パスワードのハッシュ化
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

    pub async fn find_user(&self, token: Token) -> Result<User, UserServiceError> {
        let user_id = self.token_service.verify(token)?;
        let user = self.user_repo.find_by_id(&user_id).await?;
        Ok(user)
    }

    pub async fn update_user(
        &self,
        token: Token,
        user_input: UserInput,
    ) -> Result<(), UserServiceError> {
        // トークンを持っているか検証
        let user_id = self.token_service.verify(token)?;
        let user = User {
            user_id,
            user_name: user_input.user_name,
            email: user_input.email,
            password_hash: user_input.password,
        };

        self.user_repo.update(&user).await?;
        Ok(())
    }

    pub async fn delete_user(&self, token: Token) -> Result<(), UserServiceError> {
        // トークンを持っているか検証
        let user_id = self.token_service.verify(token)?;
        self.user_repo.delete(&user_id).await?;
        Ok(())
    }
}
