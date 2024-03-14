use crate::utils::domain::User;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Post {
    title: String,
    content: String,
    shortcode: String,
    author: User,
    created_at: DateTime<Utc>,
}
