use actix_web::{web, HttpResponse};
use crate::{Pool, models::UserJson, functions::{add_user, login_user}};

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

pub async fn display_post(pool: web::Data<Pool>, path: web::Path<i32>)  -> std::io::Result<HttpResponse>{
    Ok(HttpResponse::Ok().body(format!("Path: {}", path.into_inner())))
}
