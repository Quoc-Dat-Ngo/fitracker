use actix_web::web;
use crate::controllers::user_controllers::{create_user, get_user};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
web::scope("/users")
            .route("", web::post().to(create_user))
            .route("/{id}", web::get().to(get_user))
    );
}