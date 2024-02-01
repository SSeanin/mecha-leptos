use leptos::html::{a, svg};
use leptos::svg::use_;
use leptos::*;

turf::style_sheet!("src/lib/components/atoms/link/link.scss");

#[component]
pub fn link(
    #[prop(into)] href: String,
    #[prop(into)] inner_text: String,
    #[prop(optional)] inline: bool,
) -> impl IntoView {
    a().attr("class", ClassName::LINK)
        .class(ClassName::LINK_INLINE, inline)
        .attr("href", href)
        .child(if inline {
            svg().child(use_().attr("xlink:href", "/assets/icons/sprite.svg#icon-link"))
        } else {
            svg()
        })
        .child(inner_text)
}
