use crate::entity::{claims::Claims, token::Token, user_id::UserId};

use super::service_error::token_service_error::TokenServiceError;

/// JWTやSessionIDなどのトークンの作成とトークンの検証を担うサービス
pub trait TokenService {
    /// トークンの作成
    /// JWTの場合はclaimsを使用してJWT(Base64 encoded)を返す
    /// セッションの場合はclaimsを使用してsessionID作成と、sessionIdをkeyにclaimsのデータをDBに保存
    fn create(&self, claims: Claims) -> Result<Token, TokenServiceError>;

    /// トークンの検証を行う
    /// JWTの場合は有効期限と改ざんの検知を行う
    /// セッションの場合はトークン(sessionId)をもとにDBから探して保存されていたUserIdを渡す
    fn verify(&self, token: Token) -> Result<UserId, TokenServiceError>;
}
