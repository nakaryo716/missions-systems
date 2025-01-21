use domain::entity::token::Token;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub(crate) struct JWT {
    token: String,
}

impl JWT {
    pub fn new(token: Token) -> Self {
        Self { token: token.0 }
    }
}
