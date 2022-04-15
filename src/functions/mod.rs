use crate::schema::post::dsl::*;
use crate::schema::user_info::dsl::*;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use diesel::RunQueryDsl;
use r2d2::PooledConnection;

use crate::models::{NewUserInfo, UserInfo, UserJson};

//TODO: error management
pub fn add_user(
    db_pool: PooledConnection<ConnectionManager<PgConnection>>,
    new_user: UserJson,
) -> UserInfo {
    //construct NewUser using UserJson and inserts it into the database

    let new_user_info = NewUserInfo {
        name: &new_user.name,
        password: &new_user.password,
    };

    match search_user(&db_pool, &new_user) {
        Some(already_present) => return already_present,
        None => {
            //insert one and send him
            return diesel::insert_into(user_info)
                .values(&new_user_info)
                .get_result(&db_pool)
                .expect("error inserting");
        }
    }
}

pub fn login_user(
    db_pool: PooledConnection<ConnectionManager<PgConnection>>,
    find_user: UserJson,
) -> UserInfo {
    let found_user = match search_user(&db_pool, &find_user) {
        Some(found_user) => found_user,
        None => {
            return UserInfo {
                id: -1,
                name: "notfound".to_owned(),
                password: "notfound".to_owned(),
            };
        }
    };

    if found_user.password == find_user.password {
        return found_user;
    } else {
        return UserInfo {
            id: -1,
            name: "notfound".to_owned(),
            password: "notfound".to_owned(),
        };
    }
}

fn search_user(
    db_pool: &PooledConnection<ConnectionManager<PgConnection>>,
    search_user: &UserJson,
) -> Option<UserInfo> {
    match user_info.filter(name.eq(&search_user.name)).first(db_pool) {
        Ok(found_user) => return Some(found_user),
        Err(_) => return None,
    }
}

/*
//performs inner join on the user and post table and returns vector of all the posts return by the
//user
pub fn display_posts(
    db_pool: &PooledConnection<ConnectionManager<PgConnection>>,
    user_name: String,
) -> Vec<(String, String)> {

    let something = user_info
        .inner_join(post.on(user_id.eq(id)))
        .select((name, title))
        .load(db_pool)
        .unwrap();
}
*/
