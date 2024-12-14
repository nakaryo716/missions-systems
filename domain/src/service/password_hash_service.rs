use std::{future::Future, pin::Pin};

use super::service_error::hash_error::HashServiceError;

/// 認証関連で間接的に使用するサービス
pub trait PasswordHashService {
    /// ユーザーのパスワードをHash化するメソッド
    fn hash_password(
        &self,
        password: &str,
    ) -> Pin<Box<dyn Future<Output = Result<String, HashServiceError>> + Send + 'static>>;

    /// ユーザーが送った平文のパスワードとHash化されたパスワードで一致するか検証するメソッド
    fn verify_password(
        &self,
        password: &str,
        hash_password: &str,
    ) -> Pin<Box<dyn Future<Output = Result<(), HashServiceError>> + Send + 'static>>;
}
