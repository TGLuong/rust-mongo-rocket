use crate::{models::user_model::User, repository::mongodb_repo::MongoRepo};
use mongodb::{results::InsertOneResult, bson::oid::ObjectId, Database};
use rocket::{http::Status, serde::json::Json, State};

#[post("/user", data = "<new_user>")]
pub fn create_user(
    database: &State<MongoRepo>,
    new_user: Json<User>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = User {
        id: None,
        name: new_user.name.to_owned(),
        location: new_user.location.to_owned(),
        title: new_user.title.to_owned(),
    };
    let user_detail = database.create_user(data);
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/user/<path>")]
pub fn get_user(
    database: &State<MongoRepo>,
    path: String
) -> Result<Json<User>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    }
    let user_detail = database.get_user(&id);
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/user/<path>", data = "<new_user>")]
pub fn update_user(
    database: &State<MongoRepo>,
    path: String,
    new_user: Json<User>
) -> Result<Json<User>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    }
    let data = User {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        name: new_user.name.to_owned(),
        location: new_user.location.to_owned(),
        title: new_user.title.to_owned(),
    };
    let update_detail = database.update_user(&id, data);
    match update_detail {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_user_info = database.get_user(&id);
                return match updated_user_info {
                    Ok(user) => Ok(Json(user)),
                    Err(_) => Err(Status::InternalServerError),
                };
            } else {
                return Err(Status::NotFound);
            }
        },
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/user/<path>")]
pub fn delete_user(
    database: &State<MongoRepo>,
    path: String
) -> Result<Json<&str>, Status> {
        let id = path;
        if id.is_empty() {
            return Err(Status::BadRequest);
        }
        let result = database.delete_user(&id);
        match result {
            Ok(delete) => {
                if delete.deleted_count == 1 {
                    return Ok(Json("User successfully deleted!"));
                } else {
                    return Err(Status::NotFound);
                }
            },
            Err(_) => Err(Status::InternalServerError),
        }
}

#[get("/users")]
pub fn get_all_users(
    database: &State<MongoRepo>,
) -> Result<Json<Vec<User>>, Status> {
    let users = database.get_all_users();
    match users {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(Status::InternalServerError),
    }
}