#[macro_use] extern crate rocket;
mod create_table;
mod save_user;
mod contracts;
mod query_users;
use rocket::serde::json::Json;
use contracts::{User, SuccessResponse};
use save_user::save;
use query_users::query_all_users;
use create_table::create_database;
use rocket::http::Status;

#[get("/")]
fn index() -> &'static str {
    "Welcome to my Rust Api"
}

#[post("/create", format = "application/json", data="<input>")]
fn create(input: Json<User>) -> (Status, Json<SuccessResponse>) {
    create_database();
    let user = User {
        id: None,
        name: input.name.clone(),
        email: input.email.clone(),
        password: input.password.clone(),
    };
    save(user).unwrap();
    (Status::Created, Json(SuccessResponse {
        message: "User created successfully".to_string(),
    }))
}

#[get("/users", format = "application/json")]
fn users() -> Json<Vec<User>> {
    let users = query_all_users();
    Json(users)
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, create, users])
}