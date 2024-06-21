pub mod token;
use token::token::Token;
use std::env;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::*;
    use crypto_lib::crypto::crypto::CryptoOp;
    #[tokio::test]
    async fn test_new_token() {
        dotenvy::from_path(".env").expect("dot env error");
        let key = env::var("HMAC_KEY").expect("env variable error");
        let user_id = uuid::Uuid::new_v4();
        let expiry = Utc::now().naive_utc();
        let user_id_string= user_id.to_string();
        let token_string = CryptoOp::default().generate_token(&key, user_id_string).await.expect("token geenration failure");
        let new_token = Token::new(
            user_id,
            token_string,
            expiry
        );
        assert_eq!(new_token.get_user_id(), user_id);
        assert_eq!(new_token.get_expired_time(),expiry);
    }

    #[test]
    fn test_new_token_full() {
        let id = uuid::Uuid::new_v4();
        let user_id = uuid::Uuid::new_v4();
        let expiry = Utc::now().naive_utc();
        dotenvy::from_path(".env").expect("dot env error");
        let key = env::var("HMAC_KEY").expect("env variable error");
        let token_string = key.to_string();
        let new_token = Token::new_full(
            id,
            token_string.clone(),
            user_id,
            expiry
        );
        assert_eq!(new_token.get_user_id(), user_id);
        assert_eq!(new_token.get_expired_time(),expiry);
        assert_eq!(new_token.get_token(),token_string.as_str());
        assert_eq!(new_token.get_id(),id);
    }

}
