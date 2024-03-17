pub mod auth;
pub mod post;
pub mod response;

use crate::env;
use reqwest::{Response, StatusCode};
use serde::{Deserialize, Serialize};
use url::Url;

pub type Result<T> = std::result::Result<T, ApiError>;

#[derive(Clone, Debug, Serialize, Deserialize, thiserror::Error)]
pub enum ClientError {
    #[error("entity not found")]
    NotFound,

    #[error("bad request, client side failure")]
    BadRequest,
}

#[derive(Clone, Debug, Serialize, Deserialize, thiserror::Error)]
pub enum ApiError {
    #[error("client error")]
    Failure(ClientError),

    #[error("server error")]
    Error(String),

    #[error("request error")]
    Request,
}

impl From<reqwest::Error> for ApiError {
    fn from(value: reqwest::Error) -> Self {
        if value.is_request() {
            leptos::logging::error!("{:?}", value.without_url())
        }

        ApiError::Request
    }
}

impl From<url::ParseError> for ApiError {
    fn from(value: url::ParseError) -> Self {
        // If the app fails to parse the API URL, seems like the environment variables are not configured properly
        // This conversion panics with an error to warn the developers of improper configuration
        leptos::logging::error!("Parsing of urls failed: {:?}", value);
        leptos::logging::error!("Check the environment variables for bad url format");
        panic!()
    }
}

pub fn get_api_root() -> Result<Url> {
    Ok(Url::parse(env::MECHA_API_DOMAIN)?.join(env::MECHA_API_PATH)?)
}

pub async fn make_request(method: http::Method, path: Url) -> Result<Response> {
    let res = match method {
        http::Method::GET => reqwest::get(path).await?,
        _ => unimplemented!("The passed request method is not implemented"),
    };

    if res.status().is_success() {
        Ok(res)
    } else if res.status().is_client_error() {
        match res.status() {
            StatusCode::NOT_FOUND => Err(ApiError::Failure(ClientError::NotFound)),
            _ => Err(ApiError::Failure(ClientError::BadRequest)),
        }
    } else {
        Err(ApiError::Error(String::from("Something went wrong")))
    }
}
