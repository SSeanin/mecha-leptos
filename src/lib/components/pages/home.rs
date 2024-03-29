use leptos::*;
use web_sys::window;

use crate::{
    api::post::get_posts_paginated,
    components::{Container, ContentContainer, Footer, Header, Navbar, Stories, Summary},
};

turf::style_sheet!("src/lib/components/pages/home/home.scss");

#[component]
pub fn home() -> impl IntoView {
    // TODO The pagination should be handled using the router
    const LIMIT: i32 = 5;

    let (page, set_page) = create_signal(1);

    let posts_resource = create_resource(page, |page_value| async move {
        get_posts_paginated((page_value - 1) * LIMIT, LIMIT).await
    });

    view! {
        <Container class=ClassName::HOME_CONTAINER>
            <Navbar/>

            <Header/>

            <ContentContainer>
                <Stories on_load_click=move |_| {
                    window().unwrap().scroll_to_with_x_and_y(0f64, 550f64);
                    set_page.update(|current_page| *current_page += 1)
                }>

                    <Transition fallback=move || {
                        view! { <p>Loading...</p> }
                    }>
                        {move || {
                            posts_resource
                                .get()
                                .map(|posts_result| {
                                    view! {
                                        <ErrorBoundary fallback=|_| {
                                            view! { <pre>"Error Loading Resources"</pre> }
                                        }>

                                            {posts_result
                                                .map(|posts_vec| {
                                                    posts_vec
                                                        .into_iter()
                                                        .map(|post| {
                                                            view! {
                                                                <Summary
                                                                    title=post.title
                                                                    summary=format!("{}...", post.content)
                                                                    date=post.created_at
                                                                    post_link=format!("/posts/{}", post.shortcode)
                                                                    author=format!(
                                                                        "{} {}",
                                                                        post.author.first_name,
                                                                        post.author.last_name,
                                                                    )

                                                                    author_link="#"
                                                                    category="Technology"
                                                                    category_link="#"
                                                                    min_read=2
                                                                />
                                                            }
                                                        })
                                                        .collect_view()
                                                })}

                                        </ErrorBoundary>
                                    }
                                })
                        }}

                    </Transition>

                </Stories>
                <Footer/>
            </ContentContainer>
        </Container>
    }
}
