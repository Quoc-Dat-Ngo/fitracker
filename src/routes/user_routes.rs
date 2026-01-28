use actix_web::web;
use crate::controllers::user_controllers::{create_user, get_all_users, get_user, update_user};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
web::scope("/users")
            .route("", web::get().to(get_all_users))
            .route("", web::post().to(create_user))
            .route("/{id}", web::get().to(get_user))
            .route("/{id}", web::patch().to(update_user))
    );
}