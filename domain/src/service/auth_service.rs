use crate::{
    entity::{auth_request::AuthRequest, claims::Claims, token::Token},
    repository::user_repository::UserRepository,
};

use super::{
    password_hash_service::PasswordHashService,
    service_error::auth_service_error::AuthServiceError, token_service::TokenService,
};

/// 認証(ログイン)を行うサービス
pub struct AuthService<H, T, U>
where
    H: PasswordHashService,
    T: TokenService,
    U: UserRepository,
{
    hash_service: H,
    token_service: T,
    user_repo: U,
    /// トークンの有効期限を指定する(UNIX time)
    token_exp: usize,
}

impl<H, T, U> AuthService<H, T, U>
where
    H: PasswordHashService,
    T: TokenService,
    U: UserRepository,
{
    pub fn new(hash_service: H, token_service: T, user_repo: U, token_exp: usize) -> Self {
        Self {
            hash_service,
            token_service,
            user_repo,
            token_exp,
        }
    }

    /// クライアントから送られたemailとpasswordを元に認証を行う
    pub async fn login(&self, auth_payload: AuthRequest) -> Result<Token, AuthServiceError> {
        // emailを元にパスワードを含むユーザーデータを取得
        let repository_user_data = self.user_repo.find_by_email(&auth_payload.email).await?;
        // クライアントパスワードと保存されていたハッシュ化されたパスワードを比較(bool)
        let is_authenticated = self
            .hash_service
            .verify_password(&auth_payload.password, &repository_user_data.password_hash)
            .await?;

        // パスワードが一致した場合はトークンの作成を行う
        if is_authenticated {
            let token = self
                .token_service
                .create(Claims::new(repository_user_data.user_id, self.token_exp))?;
            Ok(token)
        } else {
            Err(AuthServiceError::WrongPassword)
        }
    }
}
