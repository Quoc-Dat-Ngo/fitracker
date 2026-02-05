use crate::models::user::{NewUser, User};
use crate::schema::users::dsl::{users};
use diesel::prelude::*;

pub trait UserRepository: Send + Sync {
    fn create(
        &self,
        new_user: NewUser,
        conn: &mut PgConnection,
    ) -> Result<User, diesel::result::Error>;

    fn get_all(&self, conn: &mut PgConnection) -> Result<Vec<User>, diesel::result::Error>;
}

pub struct DieselUserRepository;

impl UserRepository for DieselUserRepository {
    fn create(
        &self,
        new_user: NewUser,
        conn: &mut PgConnection,
    ) -> Result<User, diesel::result::Error> {
        diesel::insert_into(users)
            .values(&new_user)
            .get_result::<User>(conn)
    }

    fn get_all(&self, conn: &mut PgConnection) -> Result<Vec<User>, diesel::result::Error> {
        users::load(users, conn)
    }

}
