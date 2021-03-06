use warp::Reply;
use serde_json::json;
use warp::reject;
use chrono::Utc;

use crate::{users, WebResult};
use crate::auth::create_jwt;
use crate::auth::models::{LoginRequest, LoginResponse, Role};
use crate::environment::Environment;
use crate::error::{AppError};
use crate::users::models::{User};

pub async fn register_handler(mut _req: User, _env: Environment) -> WebResult<impl Reply> {
    // match users::service::get_user_by_email(&_req.email, _env.db()).await {
    //     Ok(None) => (),
    //     Ok(existing) => {
    //         println!("[register_handler] User {} already exists", &existing.unwrap().email);
    //         return Ok(warp::reply::json(&json!({"status":"error", "message":"Unable to complete registration, email already registered"})))
    //     },
    //     _ => (),
    // }

    let hash = _env.argon().hasher().with_password(&_req.password.unwrap()).hash().unwrap();
    _req.password = Some(hash);
    _req.role = Some(Role::User);
    _req.created_at = Some(Utc::now());
    _req.updated_at = Some(Utc::now());

    let email = _req.email.clone();
    let _res = users::service::create_user(_req, _env.db()).await.map_err(|e| reject::custom(e));
    match _res {
        Ok(()) => {
            println!("[register_handler] Registration successful: {:?}", &email);
            return Ok(warp::reply::json(&json!({"status": "success"})));
        },
        Err(_e) => {
            println!("[register_handler] Error registering user {}", &email);
            return Ok(warp::reply::json(&json!({"status":"error", "message":"Registration error"})))
        }
    }
}

pub async fn login_handler(_req: LoginRequest, _env: Environment) -> WebResult<impl Reply> {
    let user_option = match users::service::get_user_by_email(&_req.email, _env.db()).await {
        Err(_e) => {
            println!("[login_handler] Error authenticating user {:?}. {:?}", &_req.email, _e);
            return Err(warp::reject::custom(AppError::WrongCredentialsError))
        },
        Ok(existing) => existing,
        // _ => {
        //     println!("[login_handler] Error authenticating user {:?}", &_req.email);
        //     return Ok(warp::reply::json(&json!({"status":"error", "message":"Email or password unknown"})));
        // },
    };

    let user = user_option;
    let is_valid = _env
        .argon()
        .verifier()
        .with_hash(&user.password.clone().unwrap())
        .with_password(&_req.password)
        .verify()
        .or(Err(warp::reject::custom(AppError::ArgonError)))?;

    if !is_valid {
        println!("[login_handler] Invalid credentials for user {:?}", &_req.email);
        return Err(warp::reject::custom(AppError::WrongCredentialsError))
    }

    let role = &user.role.clone().unwrap();
    println!("[login_handler] Authenticated user '{}' ({})", &user.email.clone(), &role);
    let token = create_jwt(&user.id.clone().unwrap().to_string(), &Role::from_str(&role.to_string())).unwrap();
    let body = LoginResponse::from_user(user, token);
    return Ok(warp::reply::json(&body));
}
