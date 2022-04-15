use serde::{Deserialize, Serialize};
use crate::schema::user_info;
use crate::schema::post;



//for select queries
#[derive( Deserialize, Serialize, Queryable)]
pub struct UserInfo{
    pub id: i32,
    pub name: String,
    pub password: String
}

//for insertion opeartions
#[derive(Insertable)]
#[table_name="user_info"]
pub struct NewUserInfo <'a>{
    pub name: &'a str,
    pub password: &'a str
}

//for transfer between functions
#[derive(Serialize, Deserialize)]
pub struct UserJson {
    pub name: String,
    pub password: String
}

#[derive( Deserialize, Serialize, Queryable)]
pub struct Post{
    pub id: i32,
    pub title: String,
    pub content: String,
    pub user_id: i32

}

#[derive( Deserialize, Serialize, Queryable)]
pub struct PostTitles{
    pub user_id: i32,
    pub post_title: String,
}

#[derive( Deserialize, Serialize, Queryable)]
pub struct PostContents{
    pub user_id: i32,
    pub post_title: String,
    pub post_content: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name="post"]
pub struct NewPost <'a>{
    pub title: & 'a str,
    pub content: & 'a str,
    pub user_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct PostJson{
    pub title: String,
    pub content: String,
}
