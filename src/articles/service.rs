use chrono::Utc;
use mongodb::bson::{doc};
use mongodb::{Database};

use crate::Result;
use crate::articles::models::{Article, Comment};
use crate::error::{AppError};
use crate::articles::utils::{parse_articles, parse_article, article_to_doc, comment_to_doc};


pub async fn get_articles(_db: Database) -> Result<Vec<Article>> {
    let mut _cursor = _db.clone().collection("articles").find(None, None).await.map_err(|_e| { 
        println!("ERROR [get_articles] {:?}", _e);
        return AppError::DataError;
    })?;
    return parse_articles(_cursor).await;
}


pub async fn get_home_articles(_db: Database) -> Result<Vec<Article>> {
    let filter = doc!{ "in_home": true };
    let mut _cursor = _db.clone().collection("articles").find(filter, None).await.map_err(|_e| { 
        println!("ERROR [get_home_articles] {:?}", _e);
        return AppError::DataError;
    })?;
    return parse_articles(_cursor).await;
}


pub async fn get_article_by_url(_url: String, _db: Database) -> Result<Article> {
    let filter = doc! { "url": _url };
    let col = _db.clone().collection("articles");
    let mut _cursor = col.find(filter, None).await.map_err(|_e| { 
        println!("ERROR [get_article_by_url] {:?}", _e);
        return AppError::DataError;
    })?;
    return parse_article(_cursor).await;
}


pub async fn create_article(_article: &Article, _db: Database) -> Result<()> {
    let doc = article_to_doc(_article);
    let _cursor = _db.collection("articles").insert_one(doc, None).await.map_err(|_e| { 
        println!("ERROR [create_article] {:?}", _e);
        return AppError::DataError;
    })?;
    Ok(())
}


pub async fn update_article(_req: &Article, _db: Database) -> Result<()> {
    let oid = mongodb::bson::oid::ObjectId::with_string(&_req.id.clone().unwrap()).unwrap();
    let filter = doc! { "_id": oid };
    let updates = doc! { "$set": {
            "title": _req.title.clone().unwrap(),
            "url": _req.url.clone().unwrap(),
            "content": _req.content.clone().unwrap(),
            "in_home": _req.in_home.clone().unwrap(),
            "tags": _req.tags.clone().unwrap(),
            "updated_at": Utc::now(),
        }
    };
    let _cursor = _db.collection("articles").update_one(filter, updates, None).await.map_err(|_e| { 
        println!("ERROR [update_article] {:?}", _e);
        return AppError::DataError;
    })?;
    Ok(())
}


pub async fn delete_article(_id: &str, _db: Database) -> Result<()> {
    let oid = mongodb::bson::oid::ObjectId::with_string(&_id).unwrap();
    let filter = doc! { "_id": oid };
    _db.collection("articles").delete_one(filter, None).await.map_err(|_e| { 
        println!("ERROR [delete_article] {:?}", _e);
        return AppError::DataError;
    })?;
    Ok(())
}


pub async fn update_home_view(_id: String, _db: Database) -> Result<()> {
    let oid = mongodb::bson::oid::ObjectId::with_string(&_id).unwrap();
    let filter = doc! { "_id": oid };
    let mut _cursor = _db.clone().collection("articles").find(filter.clone(), None).await.map_err(|_e| { 
        println!("ERROR [update_home_view] {:?}", _e);
        return AppError::DataError;
    })?;

    let _article = parse_article(_cursor).await.unwrap();

    let updates = doc! { "$set": {
            "in_home": !_article.in_home.unwrap(),
            "updated_at": Utc::now(),
        }
    };
    let _cursor = _db.collection("articles").update_one(filter, updates, None).await.map_err(|_e| { 
        println!("ERROR [update_home_view] {:?}", _e);
        return AppError::DataError;
    })?;
    Ok(())
}


// Comments

pub async fn create_comment(_article_id: String, _comment: &Comment, _db: Database) -> Result<()> {
    let oid = mongodb::bson::oid::ObjectId::with_string(&_article_id).unwrap();
    let filter = doc! { "_id": oid };
    let updates = doc! { "$push": { "comments": comment_to_doc(_comment) } };
    let _cursor = _db.collection("articles").update_one(filter, updates, None).await.map_err(|_e| { 
        println!("ERROR [get_article_by_url] {:?}", _e);
        return AppError::DataError;
    })?;
    Ok(())
}


pub async fn delete_comment(_article_id: String, _comment_id: String, _db: Database) -> Result<()> {
    let oid = mongodb::bson::oid::ObjectId::with_string(&_article_id).unwrap();
    let filter = doc! { "_id": oid };
    let updates = doc! { "$pull": { "id": _comment_id } };
    let _cursor = _db.collection("articles").update_one(filter, updates, None).await.map_err(|_e| { 
        println!("ERROR [get_article_by_url] {:?}", _e);
        return AppError::DataError;
    })?;
    Ok(())
}
