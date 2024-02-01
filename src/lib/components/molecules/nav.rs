use leptos::*;

use crate::components::Link;

turf::style_sheet!("src/lib/components/molecules/nav/nav.scss");

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <nav class=ClassName::NAV>
            <ul class=ClassName::NAV_LIST>

                <li class=ClassName::NAV_ITEM>
                    <Link inner_text="User" href="#"/>
                </li>

                <li class=ClassName::NAV_ITEM>
                    <Link inner_text="Post" href="#"/>
                </li>

            </ul>
        </nav>
    }
}
