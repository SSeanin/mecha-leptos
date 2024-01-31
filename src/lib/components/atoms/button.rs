use leptos::html::*;
use leptos::*;

turf::style_sheet!("src/lib/components/atoms/button/button.scss");

#[component]
pub fn Button(
    href: String,
    inner_text: String,
    #[prop(optional)] cta: bool,
    #[prop(optional)] outline: bool,
) -> impl IntoView {
    a().attr("class", ClassName::BTN)
        .class(ClassName::BTN_CTA, cta)
        .class(ClassName::BTN_OUTLINE, outline)
        .attr("href", href)
        .child(inner_text)
}
