use serde::{Deserialize, Serialize};
use crate::schema::user_info;



#[derive( Deserialize, Serialize, Queryable)]
pub struct UserInfo{
    pub id: i32,
    pub name: String,
    pub password: String
}

#[derive(Insertable)]
#[table_name="user_info"]
pub struct NewUserInfo <'a>{
    pub name: &'a str,
    pub password: &'a str
}

#[derive(Serialize, Deserialize)]
pub struct UserJson {
    pub name: String,
    pub password: String
}
