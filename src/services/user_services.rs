use actix_web::web;

use crate::{
    db::pool::DbPool,
    dtos::CreateUserRequest,
    errors::AppError,
    models::user::{NewUser, User},
    repositories::user_repositories::UserRepository,
};

pub async fn create_user(
    body: CreateUserRequest,
    pool: web::Data<DbPool>,
    repo: web::Data<dyn UserRepository>,
) -> Result<User, AppError> {
    // Clone the pointers (cheap)
    let pool = pool.clone();
    let repo = repo.clone();

    // We use ? here because web::block returns Result<..., BlockingError>
    // and we now have an impl From<BlockingError> for AppError
    web::block(move || -> Result<User, AppError> {
        let mut conn = pool
            .get()
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let new_user = NewUser {
            email: body.email,
            password: body.password, // TODO use argon2 to hash the password
        };

        // This returns Result<User, diesel::result::Error>
        // The ? operator converts diesel error to AppError using our impl From
        repo.create(new_user, &mut conn).map_err(AppError::from)
    })
    .await?
    // .map_err(|e| e)
}

pub async fn get_all_users(pool: web::Data<DbPool>, repo: web::Data<dyn UserRepository>) -> Result<Vec<User>, AppError> {
    let pool = pool.clone();
    let repo = repo.clone();

    web::block(move || -> Result<Vec<User>, AppError> {
        let mut conn = pool
            .get()
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        repo.get_all(&mut conn).map_err(AppError::from)
    })
    .await?

}