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
                                                view! {
                                                    <HeadingSecondary class=ClassName::HEADER_TITLE>
                                                        <Link
                                                            class=ClassName::HEADER_LINK
                                                            inner_text=latest_post.title
                                                            href=href
                                                        />

                                                    </HeadingSecondary>

                                                    <Meta>
                                                        <span>
                                                            By
                                                            <Link
                                                                inline=true
                                                                href="#"
                                                                inner_text=format!(
                                                                    "{} {}",
                                                                    latest_post.author.first_name,
                                                                    latest_post.author.last_name,
                                                                )
                                                            />

                                                        </span>
                                                        <span>
                                                            In <Link inline=true href="#" inner_text="Technology"/>
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
