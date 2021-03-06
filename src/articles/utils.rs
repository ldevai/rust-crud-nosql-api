use mongodb::bson::{Bson, doc};
use tokio::stream::StreamExt;

use crate::articles::models::{Article, Comment};
use crate::Result;
use crate::error::{AppError};

pub async fn parse_articles(mut _cursor: mongodb::Cursor) -> Result<Vec<Article>> {
    let mut result: Vec<Article> = Vec::new();
    while let Some(doc) = _cursor.next().await {
        result.push(doc_to_article(&doc?, false)?);
    }
    Ok(result)
}


pub async fn parse_article(mut _cursor: mongodb::Cursor) -> Result<Article> {
    let doc = match _cursor.next().await.map_or(Ok(None), |v| v.map(Some))? {
        Some(doc) => doc,
        _ => return Err(AppError::ArticleNotFoundError),
    };
    let mut _article = doc_to_article(&doc, true)?;
    return Ok(_article);
}


pub fn article_to_doc(_article: &Article) -> mongodb::bson::document::Document {
    doc! {
        "title": _article.title.clone().unwrap(),
        "url": _article.url.clone().unwrap(),
        "content": _article.content.clone().unwrap(),
        "in_home": _article.in_home.clone().unwrap(),
        "tags": _article.tags.clone().unwrap(),
        "created_at": _article.created_at.clone().unwrap(),
        "updated_at": _article.updated_at.clone().unwrap(),
    }
}


pub fn doc_to_article(_doc: &mongodb::bson::document::Document, _include_content: bool) -> Result<Article> {
    let id = _doc.get_object_id("_id")?;
    let title = _doc.get_str("title")?;
    let url = _doc.get_str("url")?;
    let empty = "";
    let content = match _include_content.clone() {
        true => _doc.get_str("content")?,
        _ => &empty
    };
    let in_home = _doc.get_bool("in_home")?;
    let tags = _doc.get_array("tags")?;
    let created_at = _doc.get_datetime("created_at")?;
    let updated_at = _doc.get_datetime("updated_at")?;

    let mut comments = Vec::<Comment>::new();
    match _include_content {
        true => {
            let raw_comments = _doc.get_array("comments")?; 
            for entry in raw_comments.iter() {
                let comment_doc = &entry.as_document().unwrap();
                comments.push(doc_to_comment(&comment_doc).unwrap());
            }
        }, 
        _ => ()
    };

    let result = Article {
        id: Some(id.to_string()),
        title: Some(title.to_owned()),
        url: Some(url.to_owned()),
        content: Some(content.to_owned()),
        tags: Some(tags
            .iter()
            .filter_map(|entry| match entry {
                Bson::String(v) => Some(v.to_owned()),
                _ => None,
            })
            .collect()),
        comments: Some(comments),
        created_at: Some(*created_at),
        updated_at: Some(*updated_at),
        in_home: Some(in_home),
    };
    Ok(result)
}


pub fn comment_to_doc(_comment: &Comment) -> mongodb::bson::document::Document {
    doc! {
        "id": _comment.id.clone().unwrap(),
        "author": _comment.author.clone(),
        "email": _comment.email.clone(),
        "content": _comment.content.clone(),
        "created_at": _comment.created_at.unwrap(),
    }
}


pub fn doc_to_comment(_doc: &mongodb::bson::document::Document) -> Result<Comment> {
    let comment = Comment {
        id: Some(_doc.get_str("id")?.to_owned()),
        author: _doc.get_str("author")?.to_owned(),
        email: _doc.get_str("email")?.to_owned(),
        content: _doc.get_str("content")?.to_owned(),
        created_at: Some(*_doc.get_datetime("created_at")?)
    };
    Ok(comment)
}
