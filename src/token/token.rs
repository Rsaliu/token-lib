use serde::{de::Error, Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use chrono::{NaiveDateTime, Utc};


#[derive(Default,Debug, Clone, PartialEq, Serialize, Deserialize,Copy)]
#[serde(rename_all = "camelCase")]
#[derive(sqlx::Type)]
#[sqlx(type_name = "token_type", rename_all = "lowercase")]
pub enum TokenType{
    #[default]
    AccessToken,
    RefreshToken,
    ActivationToken
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub token_uuid: Uuid,
    pub exp: NaiveDateTime,
    pub iat: NaiveDateTime,
    pub nbf: NaiveDateTime,
}


#[derive(Debug, Deserialize, Serialize, Clone,Default)]
pub struct Token{
    id: Uuid,
    token: String,
    token_type: TokenType,
    blacklisted: bool
}



impl Token{
    pub fn new(token: String,token_type: TokenType)->Self{
        Token {id:Uuid::nil(),token,token_type,blacklisted:false}
    }
    pub fn new_full(id: Uuid,token: String,token_type: TokenType,blacklisted:bool)->Self{
        Token {id,token,token_type,blacklisted}
    }
    pub fn get_token(&self)->&str{
        self.token.as_str()
    }
    pub fn get_id(&self)->Uuid{
        self.id
    }
    pub fn get_type(&self)->TokenType{
        self.token_type
    }
    pub fn get_blacklisted(&self)->bool{
        self.blacklisted
    }
}







