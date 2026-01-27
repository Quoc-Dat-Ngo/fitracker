use actix_web::{HttpResponse, web};
use crate::models::user::{CreateUser, User};

pub async fn create_user(user: web::Json<CreateUser>) -> HttpResponse {
    // Placeholder
    let new_user = User {
        id: 1,
        email: user.email.clone(),
        password: user.password.clone(), // TODO: Hash the password the save it to database
    };

    HttpResponse::Created().json(new_user)
}

pub async fn get_user(id: web::Path<u32>) -> HttpResponse {
    // Mock user for now
    let mock_user = User {
        id: 1,
        email: String::from("quocdatngo2003@gmail.com"),
        password: String::from("dat123"),
    };

    HttpResponse::Ok().json(mock_user)
}