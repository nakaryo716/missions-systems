/// Base64エンコードされたJWTまたはセッションID
/// Cookieとして保存される
#[derive(Debug, Clone)]
pub struct Token(pub String);
