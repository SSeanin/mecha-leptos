// Importing the global styles
turf::style_sheet!("sass/main.scss");

pub mod components;
pub mod utils;

use leptos::*;

use crate::components::Nav;

#[component]
pub fn App() -> impl IntoView {
    view! { <Nav/> }
}
