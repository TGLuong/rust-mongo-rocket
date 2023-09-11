use crate::{repository::mongodb_repo::MongoRepo, models::account_model::Account};
use mongodb::results::{InsertOneResult, DeleteResult};
use rocket::{State, serde::json::Json, http::Status};

#[post("/account", data = "<new_account>")]
pub fn create_account(
    database: &State<MongoRepo>,
    new_account: Json<Account>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = Account {
        id: new_account.id.to_owned(),
        user_name: new_account.user_name.to_owned(),
        password: new_account.password.to_owned(),
        ..Default::default()
    };

    let account_detail = database.create_accout(data);
    match account_detail {
        Ok(account) => Ok(Json(account)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/account/<path>")]
pub fn get_account(
    database: &State<MongoRepo>,
    path: String
) -> Result<Json<Account>, Status> {
    let account = database.get_account(&path);
    match account {
        Ok(account) => Ok(Json(account)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/account/<path>", data = "<new_account>")]
pub fn update_accout(
    database: &State<MongoRepo>,
    path: String,
    new_account: Json<Account>
) -> Result<Json<Account>, Status> {
    let data = Account {
        id: new_account.id.to_owned(),
        user_name: new_account.user_name.to_owned(),
        password: new_account.password.to_owned(),
        ..Default::default()
    };
    let update_detail = database.update_account(&path, data);
    match update_detail {
        Ok(_) => {
            let updated_account = database
                .get_account(&path);
            match updated_account {
                Ok(account) => Ok(Json(account)),
                Err(_) => Err(Status::NotFound),
            }
        },
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/accounts")]
pub fn get_all_account(
    database: &State<MongoRepo>
) -> Result<Json<Vec<Account>>, Status> {
    let accounts = database.get_all_account();
    match accounts {
        Ok(data) => Ok(Json(data)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/account/<path>")]
pub fn delete_account(
    database: &State<MongoRepo>,
    path: String
) -> Result<Json<DeleteResult>, Status>{
    let delete_detail = database.delete_account(&path);
    match delete_detail {
        Ok(delete) => Ok(Json(delete)),
        Err(_) => Err(Status::InternalServerError),
    }
}

pub fn login() -> Result<Json<Account>, Status> {
    todo!()
}