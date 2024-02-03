use leptos::*;

turf::style_sheet!("src/lib/components/templates/container/container.scss");

#[component]
pub fn container(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    let style = format!("{} {}", ClassName::CONTAINER, class);

    view! { <div class=style>{children()}</div> }
}
