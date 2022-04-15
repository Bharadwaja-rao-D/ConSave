use crate::models::PostTitles;
use crate::schema::user_info::dsl::*;
use crate::schema::post::dsl::*;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use diesel::RunQueryDsl;
use r2d2::PooledConnection;



pub fn display_posts(
    db_pool: PooledConnection<ConnectionManager<PgConnection>>,
    user_info_id: i32,
    ) -> Vec<PostTitles>{

    let data = user_info.inner_join(post)
        .select((user_id, title))
        .filter(user_id.eq(user_info_id))
        .get_results::<PostTitles>(&db_pool).unwrap();


    return data;
}

