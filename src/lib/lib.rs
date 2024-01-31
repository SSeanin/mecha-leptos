// Importing the global styles
turf::style_sheet!("sass/main.scss");

pub mod components;
pub mod utils;

use leptos::*;

use crate::components::Link;

#[component]
pub fn App() -> impl IntoView {
    view! { <Link inner_text="Some Link".to_owned() href="a".to_owned()/> }
}
