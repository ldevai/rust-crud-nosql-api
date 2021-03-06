use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Article {
    pub id: Option<String>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub content: Option<String>,
    pub tags: Option<Vec<String>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub in_home: Option<bool>,
    pub comments: Option<Vec<Comment>>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Comment {
    pub id: Option<String>,
    pub author: String,
    pub email: String,
    pub content: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NewComment {
    pub article_id: String,
    pub author: String,
    pub email: String,
    pub content: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Tag {
    pub id: Option<String>,
    pub name: Option<String>,
    pub color: Option<String>,
}
