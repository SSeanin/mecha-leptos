use leptos::*;

use crate::components::{Button, HeadingSecondary, Link, Meta};

turf::style_sheet!("src/lib/components/templates/header/header.scss");

#[component]
pub fn header(
    #[prop(into)] post_title: String,
    #[prop(into)] post_link: String,
    #[prop(into)] author: String,
    #[prop(into)] author_link: String,
    #[prop(into)] category: String,
    #[prop(into)] category_link: String,
) -> impl IntoView {
    let post_link_clone = post_link.clone();

    view! {
        <header class=ClassName::HEADER>
            <div class=ClassName::HEADER_CONTENT>
                <HeadingSecondary class=ClassName::HEADER_TITLE>
                    <Link class=ClassName::HEADER_LINK inner_text=post_title href=post_link/>
                </HeadingSecondary>

                <Meta>
                    <span>By <Link inline=true href=author_link inner_text=author/></span>
                    <span>In <Link inline=true href=category_link inner_text=category/></span>
                </Meta>

                <Button inner_text="Read On" href=post_link_clone/>
            </div>
        </header>
    }
}
