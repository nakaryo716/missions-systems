use crate::{
    entity::{token::Token, user_level::UserLevel},
    repository::user_exp_repository::UserExpRepository,
};

use super::{
    level_convert::LevelConvert, service_error::exp_error::ExpServiceError,
    token_service::TokenService,
};

/// 経験値関連のサービス実装
#[derive(Debug, Clone)]
pub struct UserExpService<E, L, T>
where
    E: UserExpRepository,
    L: LevelConvert,
    T: TokenService,
{
    exp_repo: E,
    level_converter: L,
    token_service: T,
}

impl<E, L, T> UserExpService<E, L, T>
where
    E: UserExpRepository,
    L: LevelConvert,
    T: TokenService,
{
    // ユーザー作成時に経験値のDBテーブルに対して、ユーザーのレコードを作成(1回だけ)
    // TODO: 引数に(tx: sqlx::Transaction)をとり、ユーザー作成と経験値テーブル初期化をトランザクションで行う
    pub async fn init_exp(&self, token: Token) -> Result<(), ExpServiceError> {
        let user_id = self.token_service.verify(token)?;
        self.exp_repo.init_exp(&user_id).await?;
        Ok(())
    }

    // ユーザーの経験値をDBから取得し、Levelに変換する
    pub async fn find_with_level(&self, token: Token) -> Result<UserLevel, ExpServiceError> {
        let user_id = self.token_service.verify(token)?;
        let exp = self.exp_repo.find_by_user_id(&user_id).await?;

        // ここでレベルに変換
        // LevelConvertの実装が担っている
        let exp_with_level = UserLevel::new(exp, &self.level_converter);
        Ok(exp_with_level)
    }

    // ユーザーの経験値を追加する(delta値)
    pub async fn add_experience(
        &self,
        token: Token,
        additional_exp: i64,
    ) -> Result<(), ExpServiceError> {
        let user_id = self.token_service.verify(token)?;
        // TODO: ユーザーが持つ経験値を取得しオーバーフローしないか検証する
        //       経験値が最大であったらエラーを返す
        self.exp_repo.add_exp(&user_id, additional_exp).await?;
        Ok(())
    }
}
