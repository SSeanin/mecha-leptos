use chrono::{DateTime, Local};
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
    #[prop(into)] date: DateTime<Local>,
) -> impl IntoView {
    let before_content = title.chars().collect::<Vec<char>>()[0];

    view! {
        <header>
            <HeadingTertiary
                data_content=before_content
                class=format!("{} {}", ClassName::POST_HEADER_TITLE, ClassName::MARGIN_BOTTOM_SMALL)
            >
                <Link class=ClassName::POST_HEADER_TITLE_LINK href=post_link>
                    {title}
                </Link>
            </HeadingTertiary>

            <Meta class=ClassName::POST_HEADER_META>
                <span>
                    By <Link inline=true href=author_link>
                        {author}
                    </Link>
                </span>
                <span>
                    In <Link inline=true href=category_link>
                        {category}
                    </Link>
                </span>
                <span>{min_read} " Min Read"</span>
                <span>{date.format("%B %e, %Y").to_string()}</span>
            </Meta>
        </header>
    }
}
