use chrono::NaiveDate;
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
    #[prop(into)] date: NaiveDate,
) -> impl IntoView {
    view! {
        <article class=ClassName::SUMMARY>
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

            <div>{summary}</div>

            <ButtonGroup class=ClassName::SUMMARY_ACTION_GROUP>
                <Button cta=true href="#" inner_text="Read On"/>
                <Button outline=true href="#" inner_text="Share"/>
            </ButtonGroup>
        </article>
    }
}
