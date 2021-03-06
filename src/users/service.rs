use chrono::Utc;
use mongodb::bson::{doc};
use mongodb::{Database};

use crate::Result;
use crate::error::{AppError};
use crate::users::models::{User};
use crate::users::utils::{parse_users, parse_user, user_to_doc};


pub async fn get_user_by_id(_id: String, _db: Database) -> Result<User> {
    println!("[get_user_by_id] id {:?}", &_id);
    let oid = mongodb::bson::oid::ObjectId::with_string(&_id).unwrap();
    let filter = doc! { "_id": oid };
    let mut _cursor = _db.collection("users").find(filter, None).await.map_err(|_e| { 
        println!("ERROR [get_user_by_id] {:?}", _e);
        return AppError::DataError;
    })?;
    return parse_user(_cursor).await;
}


pub async fn get_user_by_email(email: &str, _db: Database) -> Result<User> {
    println!("[get_user_by_email] email {:?}", &email);
    let filter = doc! { "email": email };
    let mut _cursor = _db.collection("users").find(filter, None).await.map_err(|_e| { 
        println!("ERROR [get_user_by_email] {:?}", _e);
        return AppError::DataError;
    })?;
    return parse_user(_cursor).await;
}


pub async fn get_users(_db: Database) -> Result<Vec<User>> {
    let mut _cursor = _db.collection("users").find(None, None).await.map_err(|_e| { 
        println!("ERROR [get_user_by_email] {:?}", _e);
        return AppError::DataError;
    })?;
    return parse_users(_cursor).await;
}


pub async fn create_user(_req: User, _db: Database) -> Result<()> {
    let doc = user_to_doc(&_req);
    let _cursor = _db.collection("users").insert_one(doc, None).await.map_err(|_e| { 
        println!("ERROR [create_user] {:?}", _e);
        return AppError::DataError;
    })?;
    Ok(())
}


pub async fn update_user(_req: User, _db: Database) -> Result<()> {
    println!("[update_user] Searching user id={}, name={}", &_req.id.clone().unwrap(), &_req.name);
    let oid = mongodb::bson::oid::ObjectId::with_string(&_req.id.unwrap()).unwrap();

    let role = &_req.role.ok_or(AppError::DataError)?;

    let filter = doc! { "_id": oid };
    let updates = doc! { "$set": {
        "email": &_req.email,
        "name": &_req.name,
        "role": &role.to_string(),
        "email": &_req.email,
        "updated_at": Utc::now()}
        };
    let _cursor = _db.collection("users").update_one(filter, updates, None).await.map_err(|_e| { 
        println!("ERROR [update_user] {:?}", _e);
        return AppError::DataError;
    })?;
    Ok(())
}


pub async fn update_user_password(_req: User, _db: Database) -> Result<()> {
    println!("[update_user_password] Searching user id={}, name={}", &_req.id.clone().unwrap(), &_req.name);
    let oid = mongodb::bson::oid::ObjectId::with_string(&_req.id.unwrap()).unwrap();
    let password = &_req.password.ok_or(AppError::DataError)?;

    let filter = doc! { "_id": oid };
    let updates = doc! { "$set": {
        "password": &password,
        "updated_at": Utc::now()}
        };
    let _cursor = _db.collection("users").update_one(filter, updates, None).await.map_err(|_e| { 
        println!("ERROR [update_user_password] {:?}", _e);
        return AppError::DataError;
    })?;
    Ok(())
}
