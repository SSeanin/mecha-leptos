use crate::utils::domain::User;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Post {
    pub title: String,
    pub content: String,
    pub shortcode: String,
    pub author: User,
    pub created_at: DateTime<Local>,
}
