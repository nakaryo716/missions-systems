use std::{
    fs::{exists, File},
    io::{Read, Write},
    sync::LazyLock,
};

use base64::{prelude::BASE64_STANDARD, Engine};
use domain::{
    entity::{claims::Claims, token::Token, user_id::UserId},
    service::{service_error::token_service_error::TokenServiceError, token_service::TokenService},
};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rand_core::{OsRng, RngCore};

static KEY_PATH: &str = "../jwt_key.txt";

static KEY: LazyLock<String> = LazyLock::new(|| {
    if !exists(KEY_PATH).expect("cannot check key file existence") {
        let key_string = generate_key();
        create_key_file(&key_string).expect("cannot create key file");
        key_string
    } else {
        read_key_from_file()
    }
});

fn read_key_from_file() -> String {
    let mut file = File::open(KEY_PATH).expect("cannot open key file");
    let mut key = String::new();
    file.read_to_string(&mut key).expect("cannot read key file");
    key
}

fn generate_key() -> String {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);
    BASE64_STANDARD.encode(key)
}

fn create_key_file(key: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(KEY_PATH)?;
    file.write_all(key.as_bytes())?;
    Ok(())
}

#[derive(Debug, Clone)]
pub struct TokenServiceImpl;

impl TokenService for TokenServiceImpl {
    fn create(&self, claims: Claims) -> Result<Token, TokenServiceError> {
        let header = Header::new(Algorithm::HS512);
        let key = &KEY;
        let token = encode(&header, &claims, &EncodingKey::from_secret(key.as_bytes()))
            .map_err(map_jwt_error)?;
        Ok(Token(token))
    }

    fn verify(&self, token: Token) -> Result<UserId, TokenServiceError> {
        let key = &KEY;
        let token = decode::<Claims>(
            &token.0,
            &DecodingKey::from_secret(key.as_bytes()),
            &Validation::new(Algorithm::HS512),
        )
        .map_err(map_jwt_error)?;
        Ok(token.claims.user_id)
    }
}

fn map_jwt_error(error: jsonwebtoken::errors::Error) -> TokenServiceError {
    match error.kind() {
        jsonwebtoken::errors::ErrorKind::InvalidToken => {
            TokenServiceError::TokenInvalid("Invalid token".to_string())
        }
        jsonwebtoken::errors::ErrorKind::InvalidIssuer => {
            TokenServiceError::ClaimsValidationError("Invalid issuer".to_string())
        }
        jsonwebtoken::errors::ErrorKind::InvalidAudience => {
            TokenServiceError::ClaimsValidationError("Invalid audience".to_string())
        }
        jsonwebtoken::errors::ErrorKind::InvalidSubject => {
            TokenServiceError::ClaimsValidationError("Invalid subject".to_string())
        }
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => TokenServiceError::TokenExpired,
        jsonwebtoken::errors::ErrorKind::InvalidAlgorithm => {
            TokenServiceError::SigningError("Invalid algorithm".to_string())
        }
        jsonwebtoken::errors::ErrorKind::Base64(err) => {
            TokenServiceError::EncodingError(format!("Base64 error: {}", err))
        }
        jsonwebtoken::errors::ErrorKind::Json(err) => {
            TokenServiceError::EncodingError(format!("JSON error: {}", err))
        }
        jsonwebtoken::errors::ErrorKind::Utf8(err) => {
            TokenServiceError::EncodingError(format!("UTF-8 error: {}", err))
        }
        _ => TokenServiceError::UnknownError("An unknown error occurred".to_string()),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_read_key_from_file() {
        let key_content = "test_key_content";
        let mut file = File::create(KEY_PATH).expect("cannot create test key file");
        file.write_all(key_content.as_bytes())
            .expect("cannot write to test key file");

        let key = read_key_from_file();
        assert_eq!(key, key_content);

        std::fs::remove_file(KEY_PATH).expect("cannot delete test key file");
    }

    #[test]
    fn test_generate_key() {
        let key = generate_key();
        let decoded_key = BASE64_STANDARD
            .decode(key.as_bytes())
            .expect("cannot decode key");
        assert_eq!(decoded_key.len(), 32);
    }

    #[test]
    fn test_create_key_file() {
        let key_content = "test_key_content";
        create_key_file(key_content).expect("cannot create key file");

        let mut file = File::open(KEY_PATH).expect("cannot open test key file");
        let mut key = String::new();
        file.read_to_string(&mut key)
            .expect("cannot read test key file");
        assert_eq!(key, key_content);

        std::fs::remove_file(KEY_PATH).expect("cannot delete test key file");
    }

    #[test]
    fn test_token_service_create() {
        let service = TokenServiceImpl;
        let claims = Claims::new(UserId("user_id".to_string()), 10000000000);

        let token = service.create(claims).expect("failed to create token");
        assert!(!token.0.is_empty());
    }

    #[test]
    fn test_token_service_verify() {
        let service = TokenServiceImpl;
        let claims = Claims::new(UserId("user_id".to_string()), 10000000000);

        let token = service
            .create(claims.clone())
            .expect("failed to create token");
        let user_id = service.verify(token).expect("failed to verify token");
        assert_eq!(user_id.0, claims.user_id.0);
    }

    #[test]
    fn test_verify_invalid_key() {
        let service = TokenServiceImpl;
        let claims = Claims::new(UserId("user_id".to_string()), 10000000000);
        let token = service.create(claims).expect("failed to create token");

        // 別のキーで検証
        let invalid_key = BASE64_STANDARD.encode(&[1u8; 32]);
        let result = decode::<Claims>(
            &token.0,
            &DecodingKey::from_secret(invalid_key.as_bytes()),
            &Validation::new(Algorithm::HS512),
        );

        assert!(result.is_err());
    }
}
