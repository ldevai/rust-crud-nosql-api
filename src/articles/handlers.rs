use warp::{Reply, reject};
use serde_json::json;
use chrono::Utc;

use crate::auth::models::AuthUser;
use crate::environment::Environment;
use crate::articles::service;
use crate::WebResult;
use crate::articles::models::{Article, NewComment, Comment};
use crate::error::{AppError};


pub async fn get_article_by_url_handler(_url: String, _env: Environment) -> WebResult<impl Reply> {
    println!("[get_article_by_url_handler] id {:?}", &_url);
    let _result = service::get_article_by_url(_url, _env.db()).await.map_err(|_e| reject::custom(AppError::DataError))?;
    Ok(warp::reply::json(&_result))
}

pub async fn get_home_articles_handler(_env: Environment) -> WebResult<impl Reply> {
    let _result = service::get_home_articles(_env.db()).await.map_err(|_e| reject::custom(AppError::DataError))?;
    Ok(warp::reply::json(&_result))
}

pub async fn get_articles_handler(_env: Environment) -> WebResult<impl Reply> {
    let _result = service::get_articles(_env.db()).await.map_err(|_e| reject::custom(AppError::DataError))?;
    Ok(warp::reply::json(&_result))
}

pub async fn create_article_handler(mut _req: Article, _env: Environment, _user: AuthUser) -> WebResult<impl Reply> {
    if _req.in_home == None {
        _req.in_home = Some(false);
    }
    if _req.tags == None {
        _req.tags = Some(Vec::new());
    }
    _req.created_at = Some(Utc::now());
    _req.updated_at = Some(Utc::now());

    println!("[create_article_handler] in_home={}", &_req.in_home.clone().unwrap());
    let _result = service::create_article(&_req, _env.db()).await.unwrap();
    println!("[create_article_handler] Created article '{}'", &_req.title.unwrap());
    Ok(warp::reply::json(&json!({"status":"success", "message":"Article saved"})))
}

pub async fn update_article_handler(mut _req: Article, _env: Environment, _user: AuthUser) -> WebResult<impl Reply> {
    if _req.in_home == None {
        _req.in_home = Some(false);
    }
    if _req.tags == None {
        _req.tags = Some(Vec::new());
    }
    println!("[update_article_handler] Updating article id={}, title={}, tags={:?}", &_req.id.clone().unwrap(), &_req.title.clone().unwrap(), &_req.tags.clone().unwrap());
    let _result = service::update_article(&_req, _env.db()).await.map_err(|_e| reject::custom(AppError::DataError))?;
    Ok(warp::reply::json(&json!({"status":"success", "message":"Article updated"})))
}

pub async fn delete_article_handler(_id: String, _env: Environment, _user: AuthUser) -> WebResult<impl Reply> {
    println!("[delete_article_handler] id={}", _id.clone());
    let _result = service::delete_article(&_id, _env.db()).await.map_err(|_e| reject::custom(AppError::DataError))?;
    Ok(warp::reply::json(&json!({"status":"success", "message":"Article deleted"})))
}

pub async fn update_home_view_handler(_id: String, _env: Environment, _user: AuthUser) -> WebResult<impl Reply> {
    println!("[update_home_view_handler] id={}", &_id);
    let _result = service::update_home_view(_id, _env.db()).await.map_err(|_e| reject::custom(AppError::DataError))?;
    Ok(warp::reply::json(&json!({"status":"success", "message":"Article updated"})))
}

// Comments

pub async fn post_comment_handler(mut _req: NewComment, _env: Environment) -> WebResult<impl Reply> {
    let comment = Comment {
        id: Some(uuid::Uuid::new_v4().to_string()),
        author: _req.author.clone(),
        email: _req.email.clone(),
        content: _req.content,
        created_at: Some(Utc::now())
    };
    let _result = service::create_comment(_req.article_id.clone(), &comment, _env.db()).await.map_err(|_e| reject::custom(AppError::DataError))?;
    println!("[post_comment_handler] article={}, email={}, name={}", _req.article_id, _req.email, _req.author);
    Ok(warp::reply::json(&json!({"status":"success", "message":"Comment saved"})))
}

pub async fn delete_comment_handler(_article_id: String, _comment_id: String, _env: Environment, _user: AuthUser) -> WebResult<impl Reply> {
    let _result = service::delete_comment(_article_id.clone(), _comment_id.clone(), _env.db()).await.map_err(|_e| reject::custom(AppError::DataError))?;
    println!("[delete_comment_handler] article_id={}, comment_id={}", _article_id, _comment_id);
    Ok(warp::reply::json(&json!({"status":"success", "message":"Comment deleted"})))
}

