use serde::{de::Error, Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use chrono::{NaiveDateTime, Utc};


#[derive(Debug, Deserialize, Serialize, Clone,Default)]
pub struct Token{
    id: Uuid,
    token: String,
    user_id: Uuid,
    expired_in: NaiveDateTime,
}



impl Token{
    pub fn new(user_id:Uuid,token: String,expired_in: NaiveDateTime)->Self{
        Token {id:Uuid::nil(),token,user_id,expired_in}
    }
    pub fn new_full(id: Uuid,token: String,user_id: Uuid,expired_in: NaiveDateTime)->Self{
        Token {id,token,user_id,expired_in}
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
}







