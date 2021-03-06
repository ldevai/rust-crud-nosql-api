use mongodb::bson::{doc};
use tokio::stream::StreamExt;
use chrono::Utc;

use crate::Result;
use crate::auth::models::{Role};
use crate::users::models::{User};
use crate::error::{AppError};


pub async fn parse_users(mut _cursor: mongodb::Cursor) -> Result<Vec<User>> {
    let mut result: Vec<User> = Vec::new();
    while let Some(doc) = _cursor.next().await {
        result.push(doc_to_user(&doc?)?);
    }
    Ok(result)
}


pub async fn parse_user(mut _cursor: mongodb::Cursor) -> Result<User> {
    let doc = match _cursor.next().await.map_or(Ok(None), |v| v.map(Some))? {
        Some(doc) => doc,
        _ => return Err(AppError::UserNotFound),
    };
    let mut _user = doc_to_user(&doc)?;
    return Ok(_user);
}


pub fn doc_to_user(doc: &mongodb::bson::document::Document) -> Result<User> {
    let id = doc.get_object_id("_id")?;
    let name = doc.get_str("name")?;
    let email = doc.get_str("email")?;
    let role = doc.get_str("role")?;
    let password = doc.get_str("password")?;
    let created_at = doc.get_datetime("created_at")?;
    let updated_at = doc.get_datetime("updated_at")?;

    let result = User {
        id: Some(id.to_string()),
        name: name.to_owned(),
        email: email.to_owned(),
        role: Some(Role::from_str(&role.to_owned())),
        password: Some(password.to_owned()),
        created_at: Some(*created_at),
        updated_at: Some(*updated_at),
    };
    Ok(result)
}


pub fn user_to_doc(_user: &User) -> mongodb::bson::document::Document {
    let doc = doc! {
    "email": _user.email.clone(),
    "name": _user.name.clone(),
    "password": _user.password.clone().unwrap(),
    "role": _user.role.clone().unwrap().to_string(),
    "created_at": _user.created_at.clone().unwrap(),
    "updated_at": Utc::now()
    };
    // let updated_at = _user.updated_at.clone();
    // match updated_at {
    //     Some(v) => doc.insert("updated_at", v),
    //     None => None
    // };
    return doc;
}
