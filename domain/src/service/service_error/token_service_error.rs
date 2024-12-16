use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum TokenServiceError {
    // トークン生成時のエラー
    #[error("failed to signin")]
    /// JWTの署名プロセス中のエラー
    SigningError(String),
    /// Base64エンコードやJSONシリアライズエラー
    #[error("token encode error")]
    EncodingError(String),
    /// Claimsが不足または無効
    #[error("claims validation error")]
    ClaimsValidationError(String),
    /// セッションストレージへの保存エラー
    #[error("session storage error")]
    StorageError(String),

    // トークン検証時のエラー
    /// トークンが有効期限切れ
    #[error("expired token")]
    TokenExpired,
    /// トークンが改ざんされている、不正な形式
    #[error("invalid token")]
    TokenInvalid(String),
    /// セッションIDやJWT内容が一致しない
    #[error("mismatch data")]
    DataMismatch(String),
    /// DBエラー（セッション検証時）
    #[error("database error")]
    DatabaseError(String),

    /// その他のエラー
    #[error("unexpected error")]
    UnknownError(String),
}
