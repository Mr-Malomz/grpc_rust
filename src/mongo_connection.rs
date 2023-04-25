use std::env;

use dotenv::dotenv;
use mongodb::{Client, Collection};

pub async fn mongo_client<T>() -> Collection<T> {
    dotenv().ok();
    let uri = match env::var("MONGOURI") {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading env variable"),
    };
    let client = Client::with_uri_str(uri)
        .await
        .expect("error connecting to database");
    let db = client.database("rustDB");

    db.collection("User")
}
