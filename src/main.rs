#[macro_use]
extern crate diesel;

mod routes;
mod controllers;
mod models;
mod db;
mod schema;
mod services;
mod repositories;
mod errors;

use actix_web::{App, HttpResponse, HttpServer, web};
use crate::routes::user_routes::init;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 3030;
    println!("Starting sever on port {port}");

    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api/v1").configure(init))
            .route("/", web::get().to(|| async {HttpResponse::Ok().json("Server is alive, hooray!")}))
    })
        .bind(("127.0.0.1", port))?
        .run()
        .await
}
