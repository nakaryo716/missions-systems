use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct UpdateUser {
    pub(crate) user_name: String,
}
