mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;
use rocket::{get, http::Status, serde::json::Json};
use api::user_api::{
    create_user, 
    get_user,
    update_user,
    delete_user,
    get_all_users
};
use api::account_api::{
    create_account,
    get_all_account,
    update_accout,
    get_account,
    delete_account,
};
use repository::mongodb_repo::MongoRepo;

#[get("/")]
fn hello() -> Result<Json<String>, Status> {
    Ok(Json(String::from("hello")))
}

#[launch]
fn rocket() -> _ {
    let database = MongoRepo::init();
    rocket::build()
        .manage(database)
        .mount("/", routes![
            hello,
            create_user,
            get_user,
            update_user,
            delete_user,
            get_all_users
        ])
        .mount("/", routes![
            get_all_account,
            create_account,
            update_accout,
            get_account,
            delete_account,
        ])
}



