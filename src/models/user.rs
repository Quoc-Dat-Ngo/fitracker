use serde::{Deserialize, Serialize};
use crate::schema::users;

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub email: String,
    pub password: String,
}

#[derive(AsChangeset, Debug, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub email: String,
    pub password: String,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct User {
    pub user_id: i32,
    pub email: String,
    pub password: String,
}

