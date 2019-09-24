use actix_web::{http, HttpResponse};
use failure::Fail;
use mongodb;
use serde_json;

use std::env;

#[derive(Fail, Debug)]
pub enum ApiError {
    #[fail(display = "configuration error: {}", _0)]
    ConfigError(String),
    #[fail(display = "database error: {}", _0)]
    MongoError(String),
    #[fail(display = "serialisation error")]
    SerialisationError,
}

impl actix_web::error::ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::ConfigError(message) | ApiError::MongoError(message) => {
                HttpResponse::with_body(http::StatusCode::INTERNAL_SERVER_ERROR, message.into())
            }
            ApiError::SerialisationError => HttpResponse::with_body(
                http::StatusCode::INTERNAL_SERVER_ERROR,
                self.to_string().into(),
            ),
        }
    }
}

impl From<env::VarError> for ApiError {
    fn from(e: env::VarError) -> Self {
        let env_name = e.to_string();
        ApiError::ConfigError(env_name)
    }
}

impl From<mongodb::error::Error> for ApiError {
    fn from(e: mongodb::error::Error) -> Self {
        let message = e.to_string();
        ApiError::MongoError(message)
    }
}

impl From<serde_json::error::Error> for ApiError {
    fn from(_: serde_json::error::Error) -> Self {
        ApiError::SerialisationError
    }
}
