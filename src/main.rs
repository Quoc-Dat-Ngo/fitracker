#[macro_use]
extern crate diesel;

mod controllers;
mod db;
mod dtos;
mod errors;
mod models;
mod repositories;
mod routes;
mod schema;
mod services;

use std::sync::Arc;

use crate::{
    db::pool::set_up_pool,
    repositories::user_repositories::{DieselUserRepository, UserRepository},
    routes::user_routes::init,
};
use actix_web::{App, HttpResponse, HttpServer, web};
use serde_json::json;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 3030;
    println!("Starting sever on port {port}");

    let pool = set_up_pool();

    let repo: Box<dyn UserRepository> = Box::new(DieselUserRepository);
    let repo_data: web::Data<dyn UserRepository> = web::Data::from(Arc::from(repo));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // Inject pool
            .app_data(repo_data.clone()) // Inject repo
            .service(web::scope("/api/v1").configure(init))
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().json(json!({ "message": "Server is alive, hooray!"})) }),
            )
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
