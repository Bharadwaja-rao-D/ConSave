use crate::models::{UserInfo, UserJson, NewUserInfo};
use crate::Pool;
use crate::schema::user_info;

//TODO: error management
pub fn add_user(db_pool: Pool, new_user: UserJson) -> UserInfo{
    //construct NewUser using UserJson and inserts it into the database

    let new_user = NewUserInfo{name: &new_user.name, password: &new_user.password };

}
