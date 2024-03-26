use leptos::*;

turf::style_sheet!("src/lib/components/atoms/heading-tertiary/heading_tertiary.scss");

#[component]
pub fn heading_tertiary(
    children: Children,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] data_content: String,
) -> impl IntoView {
    view! {
        <h3 attr:data-content=data_content class=format!("{} {}", ClassName::HEADING_TERTIARY, class)>
            {children()}
        </h3>
    }
}
