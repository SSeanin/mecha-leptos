pub mod components;

use leptos::*;

use crate::components::atoms::button::Button;

#[component]
pub fn App() -> impl IntoView {
    view! { <Button inner_text={"Something".to_owned()} href={"a".to_owned()} /> }
}
