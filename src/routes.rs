use actix_web::{web, HttpResponse};
use crate::{Pool, models::UserJson};

pub async fn index()  -> std::io::Result<HttpResponse>{
    Ok(HttpResponse::Ok().body("Index page"))
}

pub async fn signin(pool: web::Data<Pool>, user_info: web::Json<UserJson>)  -> std::io::Result<HttpResponse>{
    Ok(HttpResponse::Ok().body("SignIn page"))
}

pub async fn signup(pool: web::Data<Pool>, user_info: web::Json<UserJson>)  -> std::io::Result<HttpResponse>{
    Ok(HttpResponse::Ok().body("Signup page"))
}

pub async fn display_post(pool: web::Data<Pool>, path: web::Path<i32>)  -> std::io::Result<HttpResponse>{
    Ok(HttpResponse::Ok().body(format!("Path: {}", path.into_inner())))
}
