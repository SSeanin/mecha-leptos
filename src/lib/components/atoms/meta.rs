use leptos::*;

turf::style_sheet!("src/lib/components/atoms/meta/meta.scss");

#[component]
pub fn meta(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    view! { <div class=format!("{} {}", ClassName::META, class)>{children()}</div> }
}
