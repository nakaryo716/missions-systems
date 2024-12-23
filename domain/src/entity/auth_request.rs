use serde::Deserialize;

/// ユーザー認証でクライアントから送られるPayload
#[derive(Debug, Clone, Deserialize)]
pub struct AuthRequest {
    pub email: String,
    pub password: String,
}
