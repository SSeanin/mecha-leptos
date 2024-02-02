use leptos::*;

turf::style_sheet!("src/lib/components/templates/container/container.scss");

pub enum ContainerType {
    RAW,
    HOME,
}

impl Default for ContainerType {
    fn default() -> Self {
        ContainerType::RAW
    }
}

#[component]
pub fn container(
    children: Children,
    #[prop(optional)] container_type: ContainerType,
) -> impl IntoView {
    let style = format!(
        "{} {}",
        ClassName::CONTAINER,
        match container_type {
            ContainerType::RAW => "",
            ContainerType::HOME => ClassName::CONTAINER_HOME,
        }
    );

    view! { <div class=style>{children()}</div> }
}
