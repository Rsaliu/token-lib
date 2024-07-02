pub mod token;
use token::token::Token;
use std::env;

#[cfg(test)]
mod tests {
    use token::token::TokenType;

    use super::*;
    #[tokio::test]
    async fn test_new_token() {
        dotenvy::from_path(".env").expect("dot env error");
        let token_string = "random token string".to_string();
        let new_token = Token::new(
            token_string.clone(),
            TokenType::AccessToken
        );
        assert_eq!(new_token.get_blacklisted(), false);
        assert_eq!(new_token.get_token(),token_string);
    }

    #[test]
    fn test_new_token_full() {
        let id = uuid::Uuid::new_v4();
        let token_string = "random token string".to_string();
        let new_token = Token::new_full(
            id,
            token_string.clone(),
            TokenType::AccessToken,
            false
        );
        assert_eq!(new_token.get_blacklisted(), false);
        assert_eq!(new_token.get_token(),token_string);
        assert_eq!(new_token.get_token(),token_string.as_str());
        assert_eq!(new_token.get_id(),id);
    }

}
