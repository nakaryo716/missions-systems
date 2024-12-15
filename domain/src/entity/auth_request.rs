/// ユーザー認証でクライアントから送られるPayload
#[derive(Debug, Clone)]
pub struct AuthRequest {
    pub email: String,
    pub password: String,
}
