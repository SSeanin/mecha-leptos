use crate::{
    api::{make_request, response::JSendApiResponse, Result},
    domain::Post,
};
use reqwest::Url;

fn get_api_posts_root() -> Result<Url> {
    Ok(crate::api::get_api_root()?.join("posts")?)
}

pub async fn get_posts_paginated(skip: i32, limit: i32) -> Result<Vec<Post>> {
    let mut request_url = get_api_posts_root()?;
    request_url.set_query(Some(format!("limit={}&skip={}", limit, skip).as_str()));

    leptos::logging::log!("{:?}", request_url);

    let all_posts_res = make_request(http::Method::GET, request_url)
        .await?
        .json::<JSendApiResponse<Vec<Post>>>()
        .await?
        .into_data();

    Ok(all_posts_res.unwrap_or_default())
}
