use leptos::*;

use crate::components::Link;

turf::style_sheet!("src/lib/components/molecules/nav/nav.scss");

#[component]
pub fn nav() -> impl IntoView {
    view! {
        <nav class=ClassName::NAV>
            <ul class=ClassName::NAV_LIST>

                <li class=ClassName::NAV_ITEM>
                    <Link href="#">User</Link>
                </li>

                <li class=ClassName::NAV_ITEM>
                    <Link href="#">Post</Link>
                </li>

            </ul>
        </nav>
    }
}
