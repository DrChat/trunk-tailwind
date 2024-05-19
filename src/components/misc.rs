use yew::{function_component, html, Html, Properties};

#[derive(Default, PartialEq)]
pub enum ButtonStyle {
    #[default]
    Default,
    Neutral,
    Primary,
    Secondary,
    Accent,
    Ghost,
    Link,
}

impl ButtonStyle {
    pub fn class(&self) -> Option<&'static str> {
        match self {
            ButtonStyle::Default => None,
            ButtonStyle::Neutral => Some("btn-neutral"),
            ButtonStyle::Primary => Some("btn-primary"),
            ButtonStyle::Secondary => Some("btn-secondary"),
            ButtonStyle::Accent => Some("btn-accent"),
            ButtonStyle::Ghost => Some("btn-ghost"),
            ButtonStyle::Link => Some("btn-link"),
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct ButtonProps {
    pub label: String,
    #[prop_or_default]
    pub style: ButtonStyle,
}

/// A basic button.
#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let classes = ["btn"]
        .into_iter()
        .chain(props.style.class())
        .collect::<Vec<_>>()
        .join(" ");

    html! {
        <button class={classes}>{&props.label}</button>
    }
}
