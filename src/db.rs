use crate::error::ApiError;
use mongodb::db::ThreadedDatabase;
use mongodb::{Client, ClientOptions, ThreadedClient};
use once_cell::sync::OnceCell;

use std::env;

static MONGODB_CLIENT: OnceCell<Client> = OnceCell::new();

pub fn get_client() -> Result<&'static Client, ApiError> {
    match MONGODB_CLIENT.get() {
        Some(client) => Ok(client),
        None => {
            let mongodb_connection = env::var("MONGODB_CONNECTION")?;
            let client = Client::with_uri_and_options(
                &mongodb_connection,
                ClientOptions::with_unauthenticated_ssl(Some("/etc/ssl/cert.pem"), true),
            )?;
            auth_client(&client)?;
            MONGODB_CLIENT.set(client).unwrap();
            Ok(MONGODB_CLIENT.get().unwrap())
        }
    }
}

pub fn auth_client(client: &'_ Client) -> Result<(), ApiError> {
    let db = client.db("admin");
    db.auth(
        &env::var("MONGODB_USERNAME")?,
        &env::var("MONGODB_PASSWORD")?,
    )?;
    Ok(())
}
