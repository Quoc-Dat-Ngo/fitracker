#[macro_use]
extern crate diesel;

mod routes;
mod controllers;
mod models;
mod db;
mod schema;

use actix_web::{App, HttpServer};
use crate::routes::user_routes::init;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    println!("Starting sever on port {port}");

    HttpServer::new(|| {
        App::new()
            .configure(init)
    })
        .bind(("127.0.0.1", port))?
        .run()
        .await
}
