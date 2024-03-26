use leptos::*;

turf::style_sheet!("src/lib/components/atoms/heading-secondary/heading_secondary.scss");

#[component]
pub fn heading_secondary(
    children: Children,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] data_content: String,
) -> impl IntoView {
    view! {
        <h2 attr:data-content=data_content class=format!("{} {}", ClassName::HEADING_SECONDARY, class)>
            {children()}
        </h2>
    }
}
