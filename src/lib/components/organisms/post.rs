use chrono::{DateTime, Local};
use leptos::*;

use crate::components::PostHeader;

#[component]
pub fn post(
    children: Children,
    #[prop(into)] title: String,
    #[prop(into)] post_link: String,
    #[prop(into)] author: String,
    #[prop(into)] author_link: String,
    #[prop(into)] category: String,
    #[prop(into)] category_link: String,
    #[prop(into)] min_read: i32,
    #[prop(into)] date: DateTime<Local>,
) -> impl IntoView {
    view! {
        <article>
            <PostHeader
                title=title
                post_link=post_link
                author=author
                author_link=author_link
                category=category
                category_link=category_link
                min_read=min_read
                date=date
            />

            <div>{children()}</div>
        </article>
    }
}
