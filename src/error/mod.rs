use thiserror::Error;
use serde::{Serialize};
use mongodb::bson;

pub mod handlers;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("invalid credentials")]
    WrongCredentialsError,
    #[error("could not hash password")]
    ArgonError,
    #[error("jwt token not valid")]
    JWTTokenError,
    #[error("jwt token creation failed")]
    JWTTokenCreationError,
    #[error("no auth header")]
    NoAuthHeaderError,
    #[error("invalid auth header")]
    InvalidAuthHeaderError,
    #[error("no permission")]
    NoPermissionError,

    #[error("data error")]
    DataError,
    
    #[error("mongodb error: {0}")]
    MongoError(#[from] mongodb::error::Error),
    #[error("could not access field in document: {0}")]
    MongoDataError(#[from] bson::document::ValueAccessError),

    #[error("user not found")]
    UserNotFound,
    #[error("article not found")]
    ArticleNotFoundError,
}
impl warp::reject::Reject for AppError {}

#[derive(Serialize, Debug)]
struct ErrorResponse {
    message: String,
    status: String,
}

#[derive(Error, Debug)]
pub enum UserError {
    #[error("could not create user")]
    CreateError,
    #[error("could not update user")]
    UpdateError,
}
impl warp::reject::Reject for UserError {}
