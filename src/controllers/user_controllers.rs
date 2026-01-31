use actix_web::{HttpResponse, web};
use crate::models::user::{NewUser, UpdateUser, User};
use crate::db::connection::establish_connection;
use crate::services::user_services::create_user;
use diesel::prelude::*;

pub async fn create_user_controller(body: web::Json<NewUser>) -> HttpResponse {
    match create_user(body.into_inner()).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(e) => HttpResponse::InternalServerError().finish(),
    }
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

pub async fn get_all_users() -> HttpResponse {
    println!("Showing all users");
    use crate::schema::users::dsl::*;

    let mut connection = establish_connection();

    let users_list: Vec<User> = users
        .load::<User>(&mut connection)
        .expect("Error loading users");

    HttpResponse::Ok().json(users_list)
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