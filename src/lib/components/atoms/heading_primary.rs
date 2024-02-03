use leptos::*;

turf::style_sheet!("src/lib/components/atoms/heading-primary/heading_primary.scss");

#[component]
pub fn heading_primary(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    view! { <h1 class=format!("{} {}", ClassName::HEADING_PRIMARY, class)>{children()}</h1> }
}
