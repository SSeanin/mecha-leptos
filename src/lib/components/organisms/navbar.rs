use leptos::*;

use crate::components::{HeadingPrimary, Link, Nav};

turf::style_sheet!("src/lib/components/organisms/navbar/navbar.scss");

#[component]
pub fn navbar(
    #[prop(optional)] admin: bool,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let nav_bar_style = format!(
        "{} {} {}",
        ClassName::NAV_BAR,
        if admin { ClassName::NAV_BAR_ADMIN } else { "" },
        class
    );

    view! {
        <div class=nav_bar_style>
            <div class=ClassName::NAV_BAR_CONTENT>
                <HeadingPrimary>
                    <Link href="/home">
                        <span>"SSeanin, "</span>
                        <span>"The Blog"</span>
                    </Link>
                </HeadingPrimary>

                <Show when=move || admin fallback=|| view! {}>
                    <Nav/>
                </Show>
            </div>
        </div>
    }
}
