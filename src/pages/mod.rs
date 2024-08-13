use leptos::*;
use leptos_router::{Outlet, A};

use crate::gitlab::Gitlab;
pub use login::*;
pub use merge_requests::*;

mod login;
mod merge_requests;

#[component]
pub fn Layout(gitlab: Gitlab, #[prop(into)] logout: Callback<()>) -> impl IntoView {
    provide_context(Signal::from(move || gitlab.clone()));

    view! {
        <nav class="navbar navbar-expand bg-body-tertiary mb-3">
            <div class="container">
                <A class="navbar-brand" href="/">
                    "GitLab Bulk Tools"
                </A>
                <button
                    class="navbar-toggler"
                    type="button"
                    data-bs-toggle="collapse"
                    data-bs-target="#navbarSupportedContent"
                    aria-controls="navbarSupportedContent"
                    aria-expanded="false"
                    aria-label="Toggle navigation"
                >
                    <span class="navbar-toggler-icon"></span>
                </button>
                <div class="collapse navbar-collapse" id="navbarSupportedContent">
                    <ul class="navbar-nav me-auto mb-2 mb-lg-0">
                        <li class="nav-item">
                            <A class="nav-link" active_class="active" href="/">
                                "Merge Requests"
                            </A>
                        </li>
                        <li class="nav-item">
                            <A class="nav-link" href="/tags">
                                "Tags"
                            </A>
                        </li>
                    </ul>
                    <div class="d-flex">
                        <button class="nav-item nav-link" on:click=move |_| logout(())>
                            "Logout"
                        </button>
                    </div>
                </div>
            </div>
        </nav>
        <div class="container">
            <Outlet/>
        </div>
    }
}
