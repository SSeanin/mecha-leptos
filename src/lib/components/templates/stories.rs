use leptos::*;
use web_sys::MouseEvent;

use crate::components::{Button, HeadingTertiary, Section};

turf::style_sheet!("src/lib/components/templates/stories/stories.scss");

#[component]
pub fn stories(
    children: Children,
    #[prop(into)] on_load_click: Callback<MouseEvent>,
) -> impl IntoView {
    view! {
        <Section>
            <div class=ClassName::STORIES_HEAD>
                <HeadingTertiary class=ClassName::STORIES_HEADING>"My Stories"</HeadingTertiary>
            </div>

            <main>{children()}</main>

            <div class=ClassName::STORIES_PAGINATION>
                <Button on:click=on_load_click inner_text="Load Older" cta=true/>
            </div>
        </Section>
    }
}
