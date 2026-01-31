use actix_web::web;
use crate::controllers::user_controllers::{create_user_controller, get_all_users, get_user, update_user};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
web::scope("/users")
            .service(
        web::resource("")
                    .get(get_all_users)
                    .post(create_user_controller)
            )
            .service(
        web::resource("/{id}")
                    .get(get_user)
                    .patch(update_user)
            )
    );
}