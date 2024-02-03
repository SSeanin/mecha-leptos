use leptos::*;

turf::style_sheet!("src/lib/components/atoms/heading-tertiary/heading_tertiary.scss");

#[component]
pub fn heading_tertiary(
    children: Children,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! { <h1 class=format!("{} {}", ClassName::HEADING_TERTIARY, class)>{children()}</h1> }
}
