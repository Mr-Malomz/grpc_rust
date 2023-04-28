use std::{env, io::Error};

use dotenv::dotenv;
use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use mongodb::{Client, Collection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub location: String,
    pub title: String,
}

pub struct DBMongo {
    col: Collection<User>,
}

impl DBMongo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri)
            .await
            .expect("error connecting to database");
        let col = client.database("rustDB").collection("User");

        DBMongo { col }
    }

    pub async fn create_user(new_user: User) -> Result<InsertOneResult, Error> {
        let db = DBMongo::init().await;
        let new_doc = User {
            id: None,
            name: new_user.name,
            location: new_user.location,
            title: new_user.title,
        };
        let user = db
            .col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating user");

        Ok(user)
    }

    pub async fn get_user(id: String) -> Result<User, Error> {
        let db = DBMongo::init().await;
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = db
            .col
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting user's detail");

        Ok(user_detail.unwrap())
    }

    pub async fn update_user(id: String, new_user: User) -> Result<UpdateResult, Error> {
        let db = DBMongo::init().await;
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_user.id,
                    "name": new_user.name,
                    "location": new_user.location,
                    "title": new_user.title
                },
        };
        let updated_doc = db
            .col
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error updating user");
        Ok(updated_doc)
    }

    pub async fn delete_user(id: String) -> Result<DeleteResult, Error> {
        let db = DBMongo::init().await;
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = db
            .col
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting user");

        Ok(user_detail)
    }

    pub async fn get_all_users() -> Result<Vec<User>, Error> {
        let db = DBMongo::init().await;
        let mut cursors = db
            .col
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of users");
        let mut users: Vec<User> = Vec::new();
        while let Some(user) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            users.push(user)
        }
        Ok(users)
    }
}
