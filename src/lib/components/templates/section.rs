use leptos::*;

turf::style_sheet!("src/lib/components/templates/section/section.scss");

#[component]
pub fn section(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    view! { <section class=format!("{} {}", ClassName::SECTION, class)>{children()}</section> }
}
