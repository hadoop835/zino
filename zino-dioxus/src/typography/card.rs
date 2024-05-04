use crate::class::Class;
use dioxus::prelude::*;

/// A fixed-width span with the custom alignment.
pub fn Card(props: CardProps) -> Element {
    let class = props.class;
    rsx! {
        div {
            class: "{class}",
            header {
                class: "card-header",
                div {
                    class: "card-header-title",
                    { props.title }
                }
            }
            section {
                class: "card-content",
                div {
                    class: "content",
                    { props.content }
                }
            }
            footer {
                class: "card-footer",
                { props.footer }
            }
        }
    }
}

/// The [`Card`] properties struct for the configuration of the component.
#[derive(Clone, PartialEq, Props)]
pub struct CardProps {
    /// The class attribute for the component.
    #[props(into, default = "card".into())]
    pub class: Class,
    /// The modal title to render within the component.
    pub title: Element,
    /// The modal content to render within the component.
    pub content: Element,
    /// The modal footer to render within the component.
    pub footer: Element,
}
