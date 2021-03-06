use warp::Reply;
use serde_json::json;
use warp::reject;
use chrono::Utc;

use crate::auth::models::{AuthUser, Role};
use crate::environment::Environment;
use crate::users::models::{User, PasswordUpdateRequest};
use crate::users::service;
use crate::{WebResult};
use crate::error::{UserError, AppError};

// Returns all users
pub async fn get_users_handler(_env: Environment, _user: AuthUser) -> WebResult<impl Reply> {
    println!("[get_users_handler] Action performed by user {} ({})", _user.id, _user.role);
    let result = service::get_users(_env.db()).await.map_err(|e| reject::custom(e))?;
    Ok(warp::reply::json(&result))
}

// Returns user with given id
pub async fn get_user_by_id_handler(_id: String, _env: Environment, _user: AuthUser) -> WebResult<impl Reply> {
    let _result = service::get_user_by_id(_id.clone(), _env.db()).await.map_err(|e| reject::custom(e))?;
    println!("[get_user_by_id_handler] id={}, email={}", _id, &_result.clone().email);
    Ok(warp::reply::json(&_result))
}

// Creates new user. Same logic as in registration service.
pub async fn user_create_handler(mut _req: User, _env: Environment, _user: AuthUser) -> WebResult<impl Reply> {
    match service::get_user_by_email(&_req.email, _env.db()).await {
        Err(_e) => (),
        Ok(existing) => {
            println!("[user_create_handler] User {} already exists", &existing.email);
            return Ok(warp::reply::json(&json!({"status":"error", "message":"Unable to create user, email already registered"})))
        },
    }

    let hash = _env.argon().hasher().with_password(&_req.password.unwrap()).hash().unwrap();
    _req.password = Some(hash);
    _req.role = Some(Role::User);
    _req.created_at = Some(Utc::now());
    _req.updated_at = Some(Utc::now());

    let email = _req.email.clone();
    match service::create_user(_req, _env.db()).await {
        Err(e) => {
            println!("[user_create_handler] Error creating user {}: {:?}", &email, e);
            return Err(warp::reject::custom(UserError::CreateError))
        },
        _ => {
            println!("[user_create_handler] User creation successful: {:?}", &email);
            return Ok(warp::reply::json(&json!({"status": "success"})));
        }
    }
}

// Updates user
pub async fn user_update_handler(_req: User, _env: Environment, _user: AuthUser) -> WebResult<impl Reply> {
    println!("[user_update_handler][{}] Updating user {}", _user, &_req.email);
    let _result = service::update_user(_req, _env.db()).await.map(|_e| UserError::UpdateError);
    Ok(warp::reply::json(&json!({"status":"success", "message":"User updated"})))
}

// Changes own or other's password if admin
pub async fn password_update_handler(mut _req: PasswordUpdateRequest, _env: Environment, _user: AuthUser) -> WebResult<impl Reply> {
    // Reject non-admins changing passwords of other users
    if _user.role != Role::Admin && _user.id != _req.id.to_string() {
        return Err(warp::reject::custom(UserError::UpdateError));
    }

    let mut user = service::get_user_by_id(_req.id, _env.db()).await.map_err(|e| reject::custom(e))?;

    let target_user_id = user.id.clone().unwrap().to_string();
    println!("[password_update_handler] Action performed by {} on {}", _user.id, target_user_id);
    // current_password is required for users/admins to change their own passwords, but allow admins change others'
    if (_user.id != user.id.clone().unwrap().to_string() && _user.role != Role::Admin) || _user.id == target_user_id {
        let is_valid = _env
            .argon()
            .verifier()
            .with_hash(&user.password.unwrap())
            .with_password(&_req.current_password)
            .verify()
            .or(Err(warp::reject::custom(UserError::UpdateError)))?;

        if !is_valid {
            return Err(warp::reject::custom(AppError::WrongCredentialsError));
        }
    }

    let hash = _env.argon().hasher().with_password(&_req.new_password).hash().unwrap();
    user.password = Some(hash);
    let _result = service::update_user_password(user, _env.db()).await.map(|_e| UserError::UpdateError);
    Ok(warp::reply::json(&json!({"status":"success", "message":"Password updated"})))
}
