use super::DataEntry;
use crate::class::Class;
use dioxus::prelude::*;
use zino_core::SharedString;

/// A control that provides a menu of data entries.
pub fn DataSelect<T: DataEntry + Clone + PartialEq>(props: DataSelectProps<T>) -> Element {
    let class = props.class;
    let fullwidth_class = Class::check("is-fullwidth", props.fullwidth);
    let options = props.options;
    let selected_value = props.selected;
    let selected_option = options
        .iter()
        .find(|entry| entry.value() == selected_value)
        .or_else(|| options.first())
        .cloned();
    let selected_key = selected_option.as_ref().map(|entry| entry.key());
    let entries = options.clone();
    rsx! {
        div {
            class: "{class} {fullwidth_class}",
            select {
                name: "{props.name}",
                required: props.required,
                onmounted: move |_event| {
                    if let Some(handler) = props.on_select.as_ref() {
                        if let Some(entry) = selected_option.as_ref() {
                            handler.call(entry.clone());
                        }
                    }
                },
                onchange: move |event| {
                    if let Some(handler) = props.on_select.as_ref() {
                        let value = event.value();
                        if let Some(entry) = entries.iter().find(|d| d.value() == value) {
                            handler.call(entry.clone());
                        }
                    }
                },
                for entry in options {
                    option {
                        key: "{entry.key()}",
                        value: "{entry.value()}",
                        selected: selected_key.as_ref().is_some_and(|key| &entry.key() == key),
                        "{entry.label()}"
                    }
                }
            }
        }
    }
}

/// The [`DataSelect`] properties struct for the configuration of the component.
#[derive(Clone, PartialEq, Props)]
pub struct DataSelectProps<T: Clone + PartialEq + 'static> {
    /// The class attribute for the component.
    #[props(into, default = "select".into())]
    pub class: Class,
    /// The data options.
    pub options: Vec<T>,
    /// The selected option value.
    #[props(into, default)]
    pub selected: SharedString,
    /// The name of the control.
    #[props(into)]
    pub name: SharedString,
    /// A flag to determine whether the control is fullwidth or not.
    #[props(default)]
    pub fullwidth: bool,
    /// A flag to determine whether the control is required or not.
    #[props(default)]
    pub required: bool,
    /// An event handler to be called when the choice is selected.
    pub on_select: Option<EventHandler<T>>,
}
