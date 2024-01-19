use tracing::{debug, info, warn};
use yew::{function_component, html, Html, Properties};
use yew_hooks::use_timeout;

#[derive(PartialEq, Properties)]
struct ToastProps {
    message: String,
}

#[function_component(Toast)]
fn toast(props: &ToastProps) -> Html {
    html! {
        <div class="toast">
            <div class="alert alert-info">
                <span>{&props.message}</span>
            </div>
        </div>
    }
}

#[derive(Default, PartialEq)]
enum ButtonStyle {
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
    fn class(&self) -> Option<&'static str> {
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
struct ButtonProps {
    label: String,
    #[prop_or_default]
    style: ButtonStyle,
}

#[function_component(Button)]
fn button(props: &ButtonProps) -> Html {
    let classes = ["btn"]
        .into_iter()
        .chain(props.style.class())
        .collect::<Vec<_>>()
        .join(" ");

    html! {
        <button class={classes}>{&props.label}</button>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <section class="h-screen grid-cols-3 grid-rows-3 grid">
                <div class="mx-auto my-auto p-5 max-w-screen-md overflow-hidden rounded-xl shadow-md row-start-2 col-start-2">
                    <div class="object-center md:flex-shrink-0">
                        <h1 class="font-bold text-lg text-blue-600">{ "Hello World!" }</h1>
                        <p>{ "This is a Yew app." }</p>
                        <Button label="Click me!" style={ButtonStyle::Primary} />
                    </div>
                </div>
            </section>
        </>
    }
}

fn main() {
    use tracing_subscriber::{
        fmt::{
            format::{FmtSpan, Pretty},
            time::UtcTime,
        },
        prelude::*,
    };

    // Set up logging to the web console.
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        // .with_timer(UtcTime::rfc_3339())
        .without_time()
        .with_writer(tracing_web::MakeConsoleWriter)
        // .with_span_events(FmtSpan::ACTIVE)
        ;
    let perf_layer = tracing_web::performance_layer().with_details_from_fields(Pretty::default());
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(perf_layer)
        .init();

    yew::Renderer::<App>::new().render();
}
