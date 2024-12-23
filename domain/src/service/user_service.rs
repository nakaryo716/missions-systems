use crate::{
    entity::{
        token::Token, user::User, user_builder::UserBuilder, user_id::UserId, user_info::UserInfo,
        user_input::UserInput,
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

    pub async fn get_user_info(&self, token: Token) -> Result<UserInfo, UserServiceError> {
        let user_id = self.token_service.verify(token)?;
        let user = self.user_repo.find_by_id(&user_id).await?;
        Ok(user.into())
    }

    /// ユーザー名を変更する
    /// emailとパスワードの変更は整合性が重要なため、別サービスとして提供する予定
    pub async fn update_user_name(
        &self,
        token: Token,
        update_user_name: String,
    ) -> Result<(), UserServiceError> {
        // トークンを持っているか検証
        let user_id = self.token_service.verify(token)?;
        let stored_user = self.user_repo.find_by_id(&user_id).await?;        
        let user = User {
            user_id: stored_user.user_id,
            user_name: update_user_name,
            email: stored_user.email,
            password_hash: stored_user.password_hash,
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
