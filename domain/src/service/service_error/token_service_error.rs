use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum TokenServiceError {
    // トークン生成時のエラー
    #[error("failed to signin")]
    SigningError(String),             // JWTの署名プロセス中のエラー
    #[error("token encode error")]
    EncodingError(String),            // Base64エンコードやJSONシリアライズエラー
    #[error("claims validation error")]
    ClaimsValidationError(String),    // Claimsが不足または無効
    #[error("session storage error")]
    StorageError(String),             // セッションストレージへの保存エラー

    // トークン検証時のエラー
    #[error("expired token")]
    TokenExpired,                     // JWTが有効期限切れ
    #[error("invalid token")]
    TokenInvalid(String),             // トークンが改ざんされている、不正な形式
    #[error("mismatch data")]
    DataMismatch(String),             // セッションIDやJWT内容が一致しない
    #[error("database error")]
    DatabaseError(String),            // DBエラー（セッション検証時）

    // その他のエラー
    #[error("unexpected error")]
    UnknownError(String),             // その他の未分類エラー
}
