use leptos::*;

use crate::components::{Container, ContentContainer, Footer, Header, Navbar};

turf::style_sheet!("src/lib/components/pages/home/home.scss");

#[component]
pub fn home() -> impl IntoView {
    view! {
        <Container class=ClassName::HOME_CONTAINER>
            <Navbar/>

            <Header
                post_title="The Dream Begins Here"
                post_link="#"
                author="Saeed Andalib"
                author_link="#"
                category="Technology"
                category_link="#"
            />

            <ContentContainer>
                <Footer/>
            </ContentContainer>
        </Container>
    }
}
