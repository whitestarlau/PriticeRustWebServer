use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub user_email: String,
    pub password_hash: String,
    pub create_time: i64,
}

#[derive(Deserialize, Validate, Serialize, Debug, Clone)]
pub struct RegUser {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RegUserResp {
    pub user_email: String,
    pub uid: i32,
}
