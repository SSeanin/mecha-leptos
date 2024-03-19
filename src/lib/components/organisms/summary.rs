use chrono::{DateTime, Local};
use leptos::*;

use crate::components::{Button, ButtonGroup, PostHeader};

turf::style_sheet!("src/lib/components/organisms/summary/summary.scss");

#[component]
pub fn summary(
    #[prop(into)] title: String,
    #[prop(into)] post_link: String,
    #[prop(into)] author: String,
    #[prop(into)] author_link: String,
    #[prop(into)] category: String,
    #[prop(into)] category_link: String,
    #[prop(into)] min_read: i32,
    #[prop(into)] summary: String,
    #[prop(into)] date: DateTime<Local>,
) -> impl IntoView {
    view! {
        <article class=ClassName::SUMMARY>
            <PostHeader
                title=title
                post_link=post_link.as_str()
                author=author
                author_link=author_link
                category=category
                category_link=category_link
                min_read=min_read
                date=date
            />

            <div inner_html=summary></div>

            <ButtonGroup class=ClassName::SUMMARY_ACTION_GROUP>
                <Button cta=true href=post_link inner_text="Read On"/>
            // <Button outline=true href="#" inner_text="Share"/>
            </ButtonGroup>
        </article>
    }
}
