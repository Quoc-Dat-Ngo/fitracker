use crate::db::connection::establish_connection;
use crate::db::pool::DbPool;
use crate::dtos::CreateUserRequest;
use crate::errors::AppError;
use crate::models::user::{UpdateUser, User};
use crate::repositories::user_repositories::UserRepository;
use crate::services::user_services::{create_user, get_all_users};
use actix_web::{HttpResponse, web};
use diesel::prelude::*;
use serde_json::json;

pub async fn create_user_controller(
    body: web::Json<CreateUserRequest>,
    pool: web::Data<DbPool>,
    repo: web::Data<dyn UserRepository>,
) -> Result<HttpResponse, AppError> {
    let user = create_user(body.into_inner(), pool, repo).await?;
    Ok(HttpResponse::Created().json(json!({
        "success": true,
        "data": user,
    })))
}

pub async fn update_user(id: web::Path<i32>, body: web::Json<UpdateUser>) -> HttpResponse {
    println!("Update user with id: {:?}", id);
    use crate::schema::users::dsl::*;

    let id = id.into_inner();
    let mut connection = establish_connection();

    let update = UpdateUser {
        email: body.email.clone(),
        password: body.password.clone(),
    };

    diesel::update(users.find(id))
        .set(&update)
        .execute(&mut connection)
        .expect("Error updating user");

    HttpResponse::Ok().json(update)
}

pub async fn get_all_users_controller(pool: web::Data<DbPool>, repo: web::Data<dyn UserRepository>) -> Result<HttpResponse, AppError> {
    let users = get_all_users(pool, repo).await?;
    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "data": users,
    })))
}

pub async fn get_user(id: web::Path<i32>) -> HttpResponse {
    println!("Showing user with given id {:?}", id);

    use crate::schema::users::dsl::*;

    let id = id.into_inner();
    let mut connection = establish_connection();

    let target_user: User = users
        .find(id)
        .first(&mut connection)
        .expect("Error retrieving user with given id {user_id}");

    HttpResponse::Ok().json(target_user)
}
