use leptos::*;

turf::style_sheet!("src/lib/components/templates/footer/footer.scss");

#[component]
pub fn footer() -> impl IntoView {
    view! {
        <footer class=ClassName::FOOTER>
            <span>
                Created with <span class="footer__heart-icon">"❤️"</span> and Fueled by
                <span class="footer__rust-icon">"🦀"</span> Rust
            </span>
        </footer>
    }
}
