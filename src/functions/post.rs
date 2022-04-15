use crate::models::{PostTitles, PostContents};
use crate::models::{NewPost, Post, PostJson};
use crate::schema::post::dsl::*;
use crate::schema::user_info::dsl::*;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use diesel::RunQueryDsl;
use r2d2::PooledConnection;

pub fn insert_post(
    db_pool: PooledConnection<ConnectionManager<PgConnection>>,
    new_post: PostJson,
    user_info_id: i32
) -> String {
    let new_post_info = NewPost {
        title: &new_post.title,
        content: &new_post.content,
        user_id: user_info_id,
    };

    match post
        .filter((title.eq(new_post.title.clone())).and(user_id.eq(new_post_info.user_id)))
        .first::<Post>(&db_pool)
    {
        Ok(_) => "A post with same title is already present".to_owned(),
        Err(_) => {
            diesel::insert_into(post)
                .values(&new_post_info)
                .execute(&db_pool).unwrap();
            return "New post added".to_owned();
        }
    }
}

pub fn display_posts(
    db_pool: PooledConnection<ConnectionManager<PgConnection>>,
    user_info_id: i32,
    post_info_id: i32,
) -> Vec<PostContents> {
    let data = user_info
        .inner_join(post)
        .select((user_id, title, content))
        .filter(user_id.eq(user_info_id).and(post_id.eq(post_info_id)))
        .get_results::<PostContents>(&db_pool)
        .unwrap();

    return data;
}

pub fn display_posts_title(
    db_pool: PooledConnection<ConnectionManager<PgConnection>>,
    user_info_id: i32,
) -> Vec<PostTitles> {
    let data = user_info
        .inner_join(post)
        .select((user_id, title))
        .filter(user_id.eq(user_info_id))
        .get_results::<PostTitles>(&db_pool)
        .unwrap();

    return data;
}
