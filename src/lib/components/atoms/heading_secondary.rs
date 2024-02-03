use leptos::*;

turf::style_sheet!("src/lib/components/atoms/heading-secondary/heading_secondary.scss");

#[component]
pub fn heading_secondary(children: Children, #[prop(optional)] class: String) -> impl IntoView {
    view! { <h2 class=format!("{} {}", ClassName::HEADING_SECONDARY, class)>{children()}</h2> }
}
