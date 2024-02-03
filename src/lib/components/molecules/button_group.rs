use leptos::*;

turf::style_sheet!("src/lib/components/molecules/button-group/button_group.scss");

#[component]
pub fn button_group(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    view! { <div class=format!("{} {}", ClassName::BUTTON_GROUP, class)>{children()}</div> }
}
