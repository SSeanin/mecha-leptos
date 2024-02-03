use leptos::*;

use crate::components::{Button, HeadingTertiary, Section};

turf::style_sheet!("src/lib/components/templates/stories/stories.scss");

#[component]
pub fn stories(children: Children) -> impl IntoView {
    view! {
        <Section class=ClassName::STORIES>
            <div class=ClassName::STORIES_HEAD>
                <HeadingTertiary class=ClassName::STORIES_HEADING>"My Stories"</HeadingTertiary>
            </div>

            <main>{children()}</main>

            <div class=ClassName::STORIES_PAGINATION>
                <Button href="#" inner_text="Load More" cta=true/>
            </div>
        </Section>
    }
}
