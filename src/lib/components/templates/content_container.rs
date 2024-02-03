use leptos::*;

turf::style_sheet!("src/lib/components/templates/content-container/content_container.scss");

#[component]
pub fn content_container(children: Children) -> impl IntoView {
    view! { <div class=ClassName::CONTENT_CONTAINER>{children()}</div> }
}
