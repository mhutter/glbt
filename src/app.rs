use codee::string::JsonSerdeCodec;
use leptos::*;
use leptos_meta::{provide_meta_context, Body};
use leptos_router::{Route, Router, Routes};
use leptos_use::{storage::use_local_storage, use_color_mode, ColorMode, UseColorModeReturn};

use crate::pages::MergeRequests;

use crate::{
    gitlab::Gitlab,
    pages::{Layout, Login},
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    // let (gitlab, set_gitlab) = create_signal(None);
    let (gitlab, set_gitlab, _) = use_local_storage::<Option<Gitlab>, JsonSerdeCodec>("glbt-state");
    let logout = move |()| set_gitlab(None);

    view! {
        <Theme/>
        <Router>
            <Routes>
                <Route
                    path="/"
                    view=move || {
                        view! {
                            <Show
                                when=move || gitlab().is_some()
                                fallback=move || view! { <Login set_gitlab/> }
                            >
                                <Layout gitlab=gitlab().unwrap() logout/>
                            </Show>
                        }
                    }
                >

                    <Route path="/" view=MergeRequests/>
                    <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>
                </Route>
            </Routes>
        </Router>
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

    view! { <Body attr:data-bs-theme=theme/> }
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
