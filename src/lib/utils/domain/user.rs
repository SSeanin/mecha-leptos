use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    first_name: String,
    last_name: String,
    email: Option<String>,
}
