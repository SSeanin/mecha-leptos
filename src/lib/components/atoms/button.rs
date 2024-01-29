use leptos::{component, view, IntoView};

#[component]
pub fn Button(href: String, inner_text: String) -> impl IntoView {
    view! { <a href=href>{inner_text}</a> }
}
