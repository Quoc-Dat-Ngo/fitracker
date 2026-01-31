use crate::{errors::AppError, models::user::{NewUser, User}, repositories::user_repositories::UserRepository};



pub async fn create_user(body: NewUser) -> Result<User, AppError> {
    // TODO: Validation + Hashing password


    UserRepository::create(&body)
}