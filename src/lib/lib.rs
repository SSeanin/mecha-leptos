pub mod components;
pub mod utils;

use leptos::*;

use crate::components::Button;

#[component]
pub fn App() -> impl IntoView {
    view! { <Button outline={true} inner_text="Something".to_owned() href="a".to_owned()/> }
}
