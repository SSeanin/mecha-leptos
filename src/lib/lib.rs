// Importing the global styles
turf::style_sheet!("sass/main.scss");

pub mod components;
pub mod utils;

use leptos::*;
use leptos_router::*;

use crate::components::Home;
use crate::components::Navbar;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Navbar/>

            <Routes>
                <Route path="/" view=Home/>
                <Route path="/home" view=Home/>
                <Route path="*any" view=|| view! { <pre>"Not Found"</pre> }/>
            </Routes>
        </Router>
    }
}
