use leptos::*;

use crate::{
    api::post::get_posts_paginated,
    components::{Button, HeadingSecondary, Link, Meta},
};

turf::style_sheet!("src/lib/components/templates/header/header.scss");

#[component]
pub fn header() -> impl IntoView {
    let latest_post_resource = create_resource(|| (), |_| get_posts_paginated(0, 1));

    view! {
        <header class=ClassName::HEADER>
            <div class=ClassName::HEADER_CONTENT>

                <Suspense fallback=|| {
                    view! { <p>"Loading..."</p> }
                }>
                    {move || {
                        latest_post_resource
                            .get()
                            .map(|post_resource_result| {
                                view! {
                                    <ErrorBoundary fallback=|_| {
                                        view! { <pre>"Error Loading Resources"</pre> }
                                    }>
                                        {post_resource_result
                                            .map(|posts| {
                                                let latest_post = posts[0].clone();
                                                let href = format!("/posts/{}", latest_post.shortcode);
                                                let before_content = latest_post
                                                    .title
                                                    .chars()
                                                    .collect::<Vec<char>>()[0];
                                                view! {
                                                    <HeadingSecondary
                                                        data_content=before_content
                                                        class=ClassName::HEADER_TITLE
                                                    >
                                                        <Link class=ClassName::HEADER_LINK href=href>
                                                            {latest_post.title}
                                                        </Link>

                                                    </HeadingSecondary>

                                                    <Meta>
                                                        <span>
                                                            By <Link inline=true href="#">
                                                                {format!(
                                                                    "{} {}",
                                                                    latest_post.author.first_name,
                                                                    latest_post.author.last_name,
                                                                )}

                                                            </Link>

                                                        </span>
                                                        <span>
                                                            In <Link inline=true href="#">
                                                                Technology
                                                            </Link>
                                                        </span>
                                                    </Meta>

                                                    <Button
                                                        inner_text="Read On"
                                                        href=format!("/posts/{}", latest_post.shortcode)
                                                    />
                                                }
                                            })}

                                    </ErrorBoundary>
                                }
                            })
                    }}

                </Suspense>

            </div>
        </header>
    }
}
