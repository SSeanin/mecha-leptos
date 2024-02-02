// Importing the global styles
turf::style_sheet!("sass/main.scss");

pub mod components;
pub mod utils;

use leptos::*;

use crate::components::Navbar;

#[component]
pub fn App() -> impl IntoView {
    view! { <Navbar admin=true/> }
}
