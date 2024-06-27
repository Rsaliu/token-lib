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
#[derive(Debug, Deserialize, Serialize, Clone,Default)]
pub struct Token{
    id: Uuid,
    token: String,
    user_id: Uuid,
    expired_in: NaiveDateTime,
    token_type: TokenType,
    blacklisted: bool
}



impl Token{
    pub fn new(user_id:Uuid,token: String,expired_in: NaiveDateTime,token_type:TokenType)->Self{
        Token {id:Uuid::nil(),token,user_id,expired_in,token_type,blacklisted:false}
    }
    pub fn new_full(id: Uuid,token: String,user_id: Uuid,expired_in: NaiveDateTime,token_type:TokenType,blacklisted:bool)->Self{
        Token {id,token,user_id,expired_in,token_type,blacklisted}
    }
    pub fn get_token(&self)->&str{
        self.token.as_str()
    }
    pub fn get_id(&self)->Uuid{
        self.id
    }
    pub fn get_user_id(&self)->Uuid{
        self.user_id
    }
    pub fn get_expired_time(&self)->NaiveDateTime{
        self.expired_in
    }
    pub fn get_type(&self)->TokenType{
        self.token_type
    }
    pub fn get_blacklisted(&self)->bool{
        self.blacklisted
    }
}







