use leptos::*;

use crate::components::{Container, ContentContainer, Footer, Navbar, RawHeader, Section};

turf::style_sheet!("src/lib/components/pages/article/article.scss");

#[component]
pub fn article() -> impl IntoView {
    view! {
        <Container class=ClassName::ARTICLE_CONTAINER>
            <Navbar/>

            <RawHeader/>

            <ContentContainer>
                <Section>
                    Lorem ipsum dolor sit amet consectetur adipisicing elit. Excepturi pariatur Lorem ipsum dolor sit amet consectetur adipisicing elit. Aperiam, veniam. Excepturi nam omnis sint ex odio dignissimos debitis repellendus similique aut accusantium. Excepturi magni alias recusandae totam incidunt nisi tempora! Lorem ipsum dolor sit amet consectetur adipisicing elit. Consequatur ut at culpa ullam, illum praesentium! A officiis rem voluptates tempora reprehenderit temporibus consequuntur, enim distinctio ducimus recusandae dolorum placeat nobis. Lorem ipsum, dolor sit amet consectetur adipisicing elit. At molestias incidunt tempora nam eaque! Ex veniam commodi enim deleniti? Iusto iure at, cum expedita sint sapiente velit provident esse officiis!Lorem cumque vel quos reiciendis veritatis commodi cum voluptate odit dignissimos ex, error molestias quis culpa incidunt iure? Fugiat, quas eum.
                </Section>
                <Footer/>
            </ContentContainer>
        </Container>
    }
}
