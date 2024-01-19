use tracing::{debug, info, warn};

use yew::{function_component, html, Html, Properties};
use yew_router::prelude::*;

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

/// A basic button.
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

/// This is the "home" page.
#[function_component(Home)]
fn home() -> Html {
    html! {
        // Create a centered box. The classes here are defined by Tailwind, and the
        // corresponding CSS is generated at compile-time.
        // Normally you'd use higher-level components from Daisy UI.
        <section class="h-screen grid-cols-3 grid-rows-3 grid">
            <div class="mx-auto my-auto p-5 max-w-screen-md overflow-hidden rounded-xl shadow-md row-start-2 col-start-2">
                <div class="object-center md:flex-shrink-0">
                    <h1 class="font-bold text-lg text-blue-600">{ "Hello World!" }</h1>
                    <p>{ "This is a Yew app." }</p>
                    <p>{ "Press " }<kbd class="kbd kbd-sm">{"F"}</kbd>{" to pay respects."}</p>
                </div>
            </div>
        </section>
    }
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
}

/// This function will render the appropriate page's contents based on the current route.
fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::About => html! { {"The quick brown fox jumped over the lazy dog."} },
    }
}

/// The top-level component for the application.
#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            // Render a drawer that links to a couple of pages.
            // https://daisyui.com/components/drawer/
            <div class="drawer drawer-open">
                <input id="drawer" type="checkbox" class="drawer-toggle" />
                <div class="drawer-content">
                    // This is the actual magic from `yew-router` that will determine the active
                    // page to be rendered, based on the current URL.
                    // https://yew.rs/docs/concepts/router
                    <Switch<Route> render={switch} />
                </div>
                <div class="drawer-side">
                    <ul class="menu p-4 w-80 min-h-full bg-base-200 text-base-content">
                        <li><Link<Route> to={Route::Home}>{"Home"}</Link<Route>></li>
                        <li><Link<Route> to={Route::About}>{"About"}</Link<Route>></li>
                    </ul>
                </div>
            </div>
        </BrowserRouter>
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
