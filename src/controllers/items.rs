use crate::db;
use crate::error::ApiError;

use actix_web::HttpRequest;
use mongodb::coll::options::FindOptions;
use mongodb::db::ThreadedDatabase;
use mongodb::ThreadedClient;
use mongodb::{bson, doc, Bson};

use std::env;

pub fn items(req: HttpRequest) -> Result<String, ApiError> {
    let client = db::get_client()?;
    let _ = db::auth_client(client);
    let coll = client
        .db(&env::var("MONGODB_DATABASE")?)
        .collection("items");

    let query_prefix = req.match_info().query("prefix");
    let find_filter = if query_prefix.is_empty() {
        doc! {
            "normalised_name": Bson::RegExp(format!("^{}",query_prefix), "i".to_owned()),
        }
    } else {
        doc! {}
    };
    let find_options = FindOptions {
        limit: Some(20),
        projection: Some(doc! {
            "name":1,
        }),
        ..Default::default()
    };

    let mut cursor = coll.find(Some(find_filter.clone()), Some(find_options.clone()))?;
    let mut documents = vec![];
    while let Some(document) = cursor.next() {
        match document {
            Ok(doc) => {
                documents.push(doc);
            }
            Err(_) => {}
        }
    }
    Ok(serde_json::to_string(&documents)?)
}
