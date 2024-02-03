use leptos::*;

turf::style_sheet!("src/lib/components/templates/header/header.scss");

#[component]
pub fn raw_header() -> impl IntoView {
    view! { <header class=ClassName::HEADER></header> }
}
