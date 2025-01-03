use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserInput {
    #[validate(length(min = 1, max = 10))]
    pub user_name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 15))]
    pub password: String,
}
