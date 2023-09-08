use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc},
    results::{InsertOneResult, UpdateResult, DeleteResult},
    sync::{Client, Collection},
};
use crate::models::user_model::User;

pub struct MongoRepo {
    user_collection: Collection<User>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };

        let client = Client::with_uri_str(uri).unwrap();
        let database = client.database("rust_db");
        let collection = database.collection::<User>("User");

        MongoRepo { user_collection: collection }
    }

    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            name: new_user.name,
            location: new_user.location,
            title: new_user.title,
        };

        let user = self
            .user_collection
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating user");
        Ok(user)
    }

    pub fn get_user(&self, id: &String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc!{"_id": obj_id};
        let user_detail = self
            .user_collection
            .find_one(filter, None)
            .ok()
            .expect("Error getting user detail!");

        Ok(user_detail.unwrap())
    }

    pub fn update_user(&self, id: &String, new_user: User) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let fiter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set": {
                "id": new_user.id,
                "name": new_user.name,
                "location": new_user.location,
                "title": new_user.title
            }
        };
        let update_doc = self
            .user_collection
            .update_one(fiter, new_doc, None)
            .ok()
            .expect("Error updating user");
        Ok(update_doc)
    }

    pub fn delete_user(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let delete_detail = self
            .user_collection
            .delete_one(filter, None)
            .ok()
            .expect("Delete one error");
        Ok(delete_detail)
    }

    pub fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let cursor = self
            .user_collection
            .find(None, None)
            .ok()
            .expect("Error get all users");
        let users = cursor.map(|doc| doc.unwrap()).collect();
        Ok(users)
    }
}