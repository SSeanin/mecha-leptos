use chrono::NaiveDate;
use leptos::*;

use crate::components::{Container, ContentContainer, Footer, Navbar, Post, RawHeader, Section};

turf::style_sheet!("src/lib/components/pages/article/article.scss");

#[component]
pub fn article() -> impl IntoView {
    view! {
        <Container class=ClassName::ARTICLE_CONTAINER>
            <Navbar/>

            <RawHeader/>

            <ContentContainer>
                <Section class=ClassName::ARTICLE_SECTION>
                    <Post
                        title="The Dream Starts Here Something Big is Coming"
                        post_link="#"
                        min_read=2
                        date=NaiveDate::from_ymd_opt(2024, 3, 2).unwrap()
                        author="Saeed Andalib"
                        author_link="#"
                        category="Technology"
                        category_link="#"
                    >
                        Something
                    </Post>
                </Section>
                <Footer/>
            </ContentContainer>
        </Container>
    }
}
