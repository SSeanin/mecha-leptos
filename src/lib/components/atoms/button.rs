use leptos::*;

turf::style_sheet!("src/lib/components/atoms/button/button.scss");

#[component]
pub fn button(
    #[prop(into)] href: String,
    #[prop(into)] inner_text: String,
    #[prop(optional)] cta: bool,
    #[prop(optional)] outline: bool,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let style = format!(
        "{} {} {} {}",
        ClassName::BTN,
        if cta { ClassName::BTN_CTA } else { "" },
        if outline { ClassName::BTN_OUTLINE } else { "" },
        class,
    );

    view! {
        <a href=href class=style>
            {inner_text}
        </a>
    }
}
