use std::{future::Future, pin::Pin, sync::LazyLock};

use argon2::{
    password_hash::{self, SaltString},
    Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version,
};
use domain::service::{
    password_hash_service::PasswordHashService, service_error::hash_error::HashServiceError,
};
use rand_core::OsRng;

// Argon2構造体は必要になった時、一度だけ生成され(Lazy)、そのあとは参照として共有で使われる(Arc)
// この構造体がインスタンス化されない限り、システムは失敗し続けるため.expect()でPanicするようにしている
static ARGON2: LazyLock<Argon2> = LazyLock::new(|| {
    let argon = Argon2::new(
        // Algorithm type
        Algorithm::Argon2id,
        // Version
        Version::V0x13,
        // Parameters(cost)
        Params::new(19_456u32, 2u32, 1u32, Some(32usize))
            .expect("Failed to initialize Argon2 parameters"),
    );
    argon
});

pub struct PasswordHashServiceImpl;

impl PasswordHashService for PasswordHashServiceImpl {
    fn hash_password(
        &self,
        password: &str,
    ) -> Pin<Box<dyn Future<Output = Result<String, HashServiceError>> + Send + 'static>> {
        let password = password.to_owned();
        Box::pin(async move {
            let f = move || {
                let argon2 = &ARGON2;
                let salt = SaltString::generate(OsRng);
                let hash_password = argon2
                    .hash_password(password.as_bytes(), &salt)
                    .map_err(|_| HashServiceError::FailedToHash)?
                    .to_string();
                Ok(hash_password)
            };

            // Hash化の処理時間を測定したところ400ms程かかっており、.awaitに到達するまでに長い時間ブロックしてしまう
            // spawn_blockingを使用して専用スレッドで計算し、ワーカースレッドをブロックしないようにする
            let hash_password = tokio::task::spawn_blocking(f)
                .await
                .map_err(|_| HashServiceError::FailedToHash)??;
            Ok(hash_password)
        })
    }

    fn verify_password(
        &self,
        password: &str,
        hash_password: &str,
    ) -> Pin<Box<dyn Future<Output = Result<bool, HashServiceError>> + Send + 'static>> {
        let password = password.to_owned();
        let hash_password = hash_password.to_owned();
        Box::pin(async move {
            let f = move || {
                let argon2 = &ARGON2;
                let hash_password = PasswordHash::new(&hash_password)
                    .map_err(|_| HashServiceError::FailedToHash)?;
                match argon2.verify_password(password.as_bytes(), &hash_password) {
                    Ok(_) => Ok(true),
                    Err(e) => match e {
                        password_hash::Error::Password => Ok(false),
                        _ => Err(HashServiceError::FailedToHash),
                    },
                }
            };

            // Hash値検証の処理時間を測定したところ400ms弱かかっており、.awaitに到達するまでに長い時間ブロックしてしまう
            // spawn_blockingを使用して専用スレッドで計算し、ワーカースレッドをブロックしないようにする
            let verify_result = tokio::task::spawn_blocking(f)
                .await
                .map_err(|_| HashServiceError::FailedToVerify)?;
            verify_result
        })
    }
}

#[cfg(test)]
mod test {
    use domain::service::{
        password_hash_service::PasswordHashService, service_error::hash_error::HashServiceError,
    };

    use super::PasswordHashServiceImpl;

    #[tokio::test]
    async fn test_same_password() -> Result<(), HashServiceError> {
        let password = "#password!";
        let hash_password = PasswordHashServiceImpl.hash_password(password).await?;
        let verify_result = PasswordHashServiceImpl
            .verify_password(password, &hash_password)
            .await?;
        assert!(verify_result);
        Ok(())
    }

    #[tokio::test]
    async fn test_wrong_password() -> Result<(), HashServiceError> {
        let password = "#password!";
        let wrong_password = "!wrong_password?";

        let hash_password = PasswordHashServiceImpl.hash_password(password).await?;
        let verify_result = PasswordHashServiceImpl
            .verify_password(&wrong_password, &hash_password)
            .await?;
        assert!(!verify_result);
        Ok(())
    }

    // レインボーテーブル攻撃に対して、SaltStringが機能しているかのテスト
    #[tokio::test]
    async fn test_different_hash() -> Result<(), HashServiceError> {
        let password = "#password!";

        let hash_password1 = PasswordHashServiceImpl.hash_password(password).await?;
        let hash_password2 = PasswordHashServiceImpl.hash_password(password).await?;
        assert_ne!(hash_password1, hash_password2);
        Ok(())
    }

    #[tokio::test]
    async fn test_invalid_hash_format() {
        let password = "#password!";
        let invalid_hash = "invalid_hash_format";

        let result = PasswordHashServiceImpl
            .verify_password(password, invalid_hash)
            .await;

        assert!(result.is_err());
        match result {
            Err(HashServiceError::FailedToHash) => {}
            _ => panic!("Unexpected error type"),
        }
    }
}
