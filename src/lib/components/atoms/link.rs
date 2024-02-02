use leptos::*;

turf::style_sheet!("src/lib/components/atoms/link/link.scss");

#[component]
pub fn link(
    #[prop(into)] href: String,
    #[prop(into)] inner_text: String,
    #[prop(optional)] inline: bool,
) -> impl IntoView {
    let style = format!(
        "{} {}",
        ClassName::LINK,
        if inline { ClassName::LINK_INLINE } else { "" }
    );

    view! {
        <a href=href class=style>
            <Show when=move || inline fallback=move || view! {}>
                <svg>
                    <use_ xlink=true href="/assets/icons/sprite.svg#icon-link"></use_>
                </svg>
            </Show>
            {inner_text}
        </a>
    }
}
