#[macro_use]
extern crate diesel;
extern crate dotenv;

use dotenv::dotenv;
use phone::get_phones_query;
use std::env;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::Pool;
use diesel::r2d2::{self, ConnectionManager};
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};


pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub mod models;
pub mod schema;
pub mod phone;

use models::*;




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Database URL is required");
    let host = env::var("HOST").expect("HOST must be setted");
    let port = env::var("PORT").expect("PORT must be setted");

  

    let connection = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool = Pool::builder()
        .build(connection)
        .expect("Error in create pool");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .service(
                web::scope("/authors")
                    .service(post_author)
                    .service(get_authors)
                    .service(get_authors2)
                    .service(get_authors3)
            )
            .service(
                web::scope("/books")
                    .service(post_book)
            )
            .service(
                web::scope("/authorBook")
                    .service(post_books_authors)
                    
            )
            .service(
                web::scope("/api")
                .service(get_phones_query)

            )

            

    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}