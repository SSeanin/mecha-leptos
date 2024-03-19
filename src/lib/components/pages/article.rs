use leptos::*;
use leptos_router::use_params;

use crate::{
    api::{params::ArticleParams, post::get_post},
    components::{Container, ContentContainer, Footer, Navbar, Post, RawHeader, Section},
};

turf::style_sheet!("src/lib/components/pages/article/article.scss");

#[component]
pub fn article() -> impl IntoView {
    let params = use_params::<ArticleParams>();
    let post_shortcode_param = params.with(|params| {
        params
            .as_ref()
            .map(|params| params.shortcode.clone())
            .unwrap_or_default()
    });

    let (shortcode, _set_shortcode) = create_signal(post_shortcode_param);

    let post_resource = create_resource(shortcode, |value| async {
        get_post(value.unwrap_or_default().as_str()).await
    });

    view! {
        <Container class=ClassName::ARTICLE_CONTAINER>
            <Navbar/>

            <RawHeader/>

            <ContentContainer>
                <Section class=ClassName::ARTICLE_SECTION>
                    <Suspense fallback=|| {
                        view! { <p>Loading...</p> }
                    }>

                        {move || {
                            post_resource
                                .get()
                                .map(|post_result| {
                                    view! {
                                        <ErrorBoundary fallback=|_| {
                                            view! { <pre>Error Loading Resources</pre> }
                                        }>
                                            {post_result
                                                .map(|post| {
                                                    view! {
                                                        <Post
                                                            title=post.title
                                                            post_link=format!("/posts/{}", post.shortcode)
                                                            min_read=2
                                                            date=post.created_at
                                                            author=format!(
                                                                "{} {}",
                                                                post.author.first_name,
                                                                post.author.last_name,
                                                            )

                                                            author_link="#"
                                                            category="Technology"
                                                            category_link="#"
                                                        >
                                                            {post.content}
                                                        </Post>
                                                    }
                                                })}

                                        </ErrorBoundary>
                                    }
                                })
                        }}

                    </Suspense>

                </Section>
                <Footer/>
            </ContentContainer>
        </Container>
    }
}
