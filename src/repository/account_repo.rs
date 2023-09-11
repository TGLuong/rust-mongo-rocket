use mongodb::results::{InsertOneResult, UpdateResult, DeleteResult};
use mongodb::bson::{
    oid::ObjectId, 
    extjson::de::Error,
    doc
};
use rocket::http::Status;
use crate::repository::mongodb_repo::MongoRepo;
use crate::models::account_model::Account;

impl MongoRepo {
    pub fn create_accout(&self, new_account: Account) -> Result<InsertOneResult, Error> {
        let create_detail = self
            .account_collection
            .insert_one(new_account, None)
            .ok()
            .expect("Error create account");
        Ok(create_detail)
    }

    pub fn update_account(&self, id: &String, new_account: Account) -> Result<UpdateResult, Error> {
        println!("id: {}", id);
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_update = doc! {
            "$set": {
                "_id": obj_id,
                "user_name": new_account.user_name,
                "password": new_account.password,
            }
        };
        let update_detail = self
            .account_collection
            .update_one(filter, new_update, None)
            .ok()
            .expect("Error update account");
        Ok(update_detail)
    }

    pub fn delete_account(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let delete_detail = self
            .account_collection
            .delete_one(filter, None)
            .ok()
            .expect("Error delete accoutn");
        Ok(delete_detail)
    }

    pub fn get_account(&self, id: &String) -> Result<Account, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};

        let account = self
            .account_collection
            .find_one(filter, None)
            .ok()
            .expect("Error get account");
        Ok(account.unwrap())
    }

    pub fn get_all_account(&self) -> Result<Vec<Account>, Error> {
        let cursor = self
            .account_collection
            .find(None, None)
            .ok()
            .expect("Error get all account");
        let accounts: Vec<Account> = cursor.map(|doc| doc.unwrap()).collect();
        Ok(accounts)
    }

    pub fn login(&self, login_info: Account) -> Result<Account, Status> {
        let filter = doc! {"username": login_info.user_name};

        let account = self
            .account_collection
            .find_one(filter, None)
            .ok()
            .expect("Error can't find account");
        
        match account {
            Some(account) => {
                if login_info.password == account.password {
                    Ok(account)
                } else {
                    Err(Status::NotFound)
                }
            },
            None => Err(Status::InternalServerError)
        }
    }
}