use leptos::*;
use leptos_router::*;

turf::style_sheet!("src/lib/components/atoms/link/link.scss");

#[component]
pub fn link(
    children: Children,
    #[prop(into)] href: String,
    #[prop(optional)] inline: bool,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let style = format!(
        "{} {} {}",
        ClassName::LINK,
        if inline { ClassName::LINK_INLINE } else { "" },
        class,
    );

    view! {
        <A href=href class=style>
            <Show when=move || inline fallback=|| view! {}>
                <svg>
                    <use_ xlink=true href="/assets/icons/sprite.svg#icon-link"></use_>
                </svg>
            </Show>
            {children()}
        </A>
    }
}
