use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::auth::models::Role;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Option<String>,
    pub email: String,
    pub name: String,
    #[serde(skip_serializing)]
    pub password: Option<String>,
    pub role: Option<Role>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct PasswordUpdateRequest {
    pub id: String,
    pub current_password: String,
    pub new_password: String,
}
