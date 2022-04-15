#[macro_use]
extern crate diesel;

mod schema;
mod models;
mod routes;
mod functions;

use actix_web::{HttpServer, App, web, guard};
use diesel::{r2d2::ConnectionManager, PgConnection};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

use routes::{index, signin, signup, display_post, display_post_titles, insert_post};


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    dotenv::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("database not found");
    //we need to use this pool in the handlers
    let db_pool = Pool::builder().build(ConnectionManager::new(db_url)).expect("Error accessing the database");


    HttpServer::new(move ||
        App::new()
        .app_data(web::Data::new(db_pool.clone()))
        .route("/", web::get().to(index))
        .service(
            web::resource("/signin")
            .guard(guard::Header("content-type", "application/json"))
            .route(web::post().to(signin))
            )
        .service(
            web::resource("/signup")
            .guard(guard::Header("content-type", "application/json"))
            .route(web::post().to(signup))
            )
        //TODO: try using scope for these handlers
        .service(
            web::resource("/post/{user_id}/insert")
            .guard(guard::Header("content-type", "application/json"))
            .route(web::post().to(insert_post))
            )
        .service(
            web::resource("/post/{user_id}")
            .guard(guard::Header("content-type", "application/json"))
            .route(web::get().to(display_post_titles))
            )
        .service(
            web::resource("/post/{user_id}/{post_id}")
            .route(web::get().to(display_post))
            )
        )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
