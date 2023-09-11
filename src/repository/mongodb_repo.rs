use std::env;
extern crate dotenv;
use dotenv::dotenv;
use mongodb::{
    sync::{Client, Collection}, results::CollectionSpecification,
};
use crate::models::user_model::User;
use crate::models::account_model::Account;

pub struct MongoRepo {
    pub user_collection: Collection<User>,
    pub account_collection: Collection<Account>,
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
        let user_collection = database.collection::<User>("User");
        let account_collection = database.collection::<Account>("Account");

        MongoRepo { 
            user_collection, 
            account_collection,
        }
    }
}