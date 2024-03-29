// Importing the global styles
turf::style_sheet!("sass/main.scss");

pub mod components;
pub mod env;
pub mod utils;

pub use utils::*;

use leptos::*;
use leptos_router::*;

use crate::components::{Article, Home};

// TODO use nested routing
#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="" view=Home/>
                <Route path="home" view=Home/>
                <Route path="/posts/:shortcode" view=Article/>
                <Route path="*any" view=|| view! { <pre>"Not Found"</pre> }/>
            </Routes>
        </Router>
    }
}
