use chrono::NaiveDate;
use leptos::*;

use crate::components::{HeadingTertiary, Link, Meta};

turf::style_sheet!("src/lib/components/molecules/post-header/post_header.scss");

#[component]
pub fn post_header(
    #[prop(into)] title: String,
    #[prop(into)] post_link: String,
    #[prop(into)] author: String,
    #[prop(into)] author_link: String,
    #[prop(into)] category: String,
    #[prop(into)] category_link: String,
    #[prop(into)] min_read: i32,
    #[prop(into)] date: NaiveDate,
) -> impl IntoView {
    view! {
        <header>
            <HeadingTertiary class=format!(
                "{} {}",
                ClassName::POST_HEADER_TITLE,
                ClassName::MARGIN_BOTTOM_SMALL,
            )>
                <Link class=ClassName::POST_HEADER_TITLE_LINK href=post_link inner_text=title/>
            </HeadingTertiary>

            <Meta class=ClassName::POST_HEADER_META>
                <span>By <Link inline=true href=author_link inner_text=author/></span>
                <span>In <Link inline=true href=category_link inner_text=category/></span>
                <span>{min_read} " Min Read"</span>
                <span>{date.format("%B %e, %Y").to_string()}</span>
            </Meta>
        </header>
    }
}
