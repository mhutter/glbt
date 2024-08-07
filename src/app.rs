use leptos::*;
use leptos_meta::{provide_meta_context, Body, Stylesheet};
use leptos_use::{use_color_mode, ColorMode, UseColorModeReturn};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Theme/>
        <div class="container">
            <h1>"I works!"</h1>
        </div>
        <Scripts/>
    }
}

#[component]
fn Theme() -> impl IntoView {
    let UseColorModeReturn { mode, .. } = use_color_mode();
    let theme = move || match mode() {
        ColorMode::Dark => "dark",
        _ => "light",
    };

    view! {
        <Body attr:data-bs-theme=theme/>
        <Stylesheet
            href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/css/bootstrap.min.css"
            attr:integrity="sha384-QWTKZyjpPEjISv5WaRU9OFeRpok6YctnYmDr5pNlyT2bRjXh0JMhjY6hW+ALEwIH"
            attr:crossorigin="anonymous"
        />
    }
}

#[component]
fn Scripts() -> impl IntoView {
    view! {
        <script
            src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/js/bootstrap.bundle.min.js"
            integrity="sha384-YvpcrYf0tY3lHB60NNkmXc5s9fDVZLESaAA55NDzOxhy9GkcIdslK1eN7N6jIeHz"
            crossorigin="anonymous"
        ></script>
    }
}
