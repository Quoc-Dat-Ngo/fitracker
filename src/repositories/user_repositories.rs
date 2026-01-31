use diesel::prelude::*;
use crate::schema::users::dsl::*;
use crate::db::connection::establish_connection;
use crate::models::user::{User, NewUser, UpdateUser};
use crate::errors::AppError;

pub struct UserRepository;

impl UserRepository {
    pub fn create(new_user: &NewUser) -> Result<User, AppError> {
        let mut conn = establish_connection();

        diesel::insert_into(users)
            .values(new_user)
            .get_result::<User>(&mut conn)
            .map_err(AppError::from)
    }

}
