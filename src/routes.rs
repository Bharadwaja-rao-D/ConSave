use actix_web::{web, HttpResponse};
use crate::{Pool, models::{UserJson, PostJson}};
use crate::functions::user::{login_user, add_user};
use crate::functions::post;

pub async fn index()  -> std::io::Result<HttpResponse>{
    Ok(HttpResponse::Ok().body("Index page"))
}

pub async fn signin(pool: web::Data<Pool>, user_info: web::Json<UserJson>)  -> std::io::Result<HttpResponse>{
    Ok(HttpResponse::Ok().json(login_user(pool.get().unwrap(), user_info.into_inner())))
}

pub async fn signup(pool: web::Data<Pool>, user_info: web::Json<UserJson>)  -> std::io::Result<HttpResponse>{
    let user =  add_user(pool.get().unwrap(), user_info.into_inner());
    return Ok(HttpResponse::Ok().json(user));
}

pub async fn display_post_titles(pool: web::Data<Pool>, path: web::Path<i32>)  -> std::io::Result<HttpResponse>{

    let titles = post::display_posts_title(pool.get().unwrap(), path.into_inner());
    return Ok(HttpResponse::Ok().json(titles));
}

pub async fn display_post(pool: web::Data<Pool>, path: web::Path<(i32, i32)>)  -> std::io::Result<HttpResponse>{
    let (post, user) = path.into_inner();
    let content = post::display_posts(pool.get().unwrap(), post, user);
    Ok(HttpResponse::Ok().json(content))
}

pub async fn insert_post(pool: web::Data<Pool>, path: web::Path<i32>, post_info: web::Json<PostJson>)  -> std::io::Result<HttpResponse>{
    Ok(HttpResponse::Ok().body(post::insert_post(pool.get().unwrap(),post_info.into_inner(), path.into_inner())))
}
