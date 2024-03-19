use leptos::*;
use leptos_router::*;

#[derive(Params, PartialEq)]
pub struct ArticleParams {
    pub shortcode: Option<String>,
}
