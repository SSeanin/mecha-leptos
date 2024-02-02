use leptos::*;

use crate::components::Nav;

turf::style_sheet!("src/lib/components/organisms/navbar/navbar.scss");

#[component]
pub fn navbar(#[prop(optional)] admin: bool) -> impl IntoView {
    let nav_bar_style = format!(
        "{} {}",
        ClassName::NAV_BAR,
        if admin { ClassName::NAV_BAR_ADMIN } else { "" }
    );

    view! {
        <div class=nav_bar_style>
            <div class=ClassName::NAV_BAR_CONTENT>
                <h1 class=ClassName::HEADING_PRIMARY>
                    <span>"SSeanin, "</span>
                    <span>"The Blog"</span>
                </h1>

                <Show when=move || admin fallback=|| view! {}>
                    <Nav/>
                </Show>
            </div>
        </div>
    }
}
