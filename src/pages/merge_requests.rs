use leptos::*;

use crate::{
    components::Error,
    gitlab::{Gitlab, MergeRequest, StateEvent},
};

#[component]
pub fn MergeRequests() -> impl IntoView {
    let gitlab = use_context::<Signal<Gitlab>>().expect("Gitlab client provided");

    let merge_requests = create_local_resource(
        || (),
        move |()| async move {
            gitlab()
                .get_open_mrs()
                .await
                .map(|mrs| mrs.into_iter().map(|mr| create_rw_signal(mr)).collect())
        },
    );

    view! {
        <h1>"Merge Requests"</h1>
        {move || match merge_requests() {
            None => view! { <p>"Loading ..."</p> }.into_view(),
            Some(Ok(merge_requests)) => view! { <MergeRequestTable merge_requests/> }.into_view(),
            Some(Err(err)) => view! { <Error err/> }.into_view(),
        }}
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CheckboxState {
    Unchecked,
    Indeterminate,
    Checked,
}

#[component]
pub fn MergeRequestTable(merge_requests: Vec<RwSignal<MergeRequest>>) -> impl IntoView {
    view! {
        <table class="table">
            <thead>
                <tr>
                    <th scope="col">"Title"</th>
                    <th scope="col">"Reference"</th>
                    <th scope="col">"MR Status"</th>
                    <th scope="col">"CI Status"</th>
                    <th scope="col">"Actions"</th>
                </tr>
            </thead>
            <tbody>
                {merge_requests
                    .into_iter()
                    .map(|mr| {
                        view! { <MergeRequest mr/> }
                    })
                    .collect_view()}
            </tbody>
        </table>
    }
}

#[component]
pub fn MergeRequest(mr: RwSignal<MergeRequest>) -> impl IntoView {
    let gitlab = use_context::<Signal<Gitlab>>().expect("Gitlab client provided");
    let (error, set_error) = create_signal(None);

    let pipeline = create_local_resource(
        || (),
        move |()| async move {
            gitlab()
                .get_latest_mr_pipelines(&mr())
                .await
                .expect("pipeline status")
        },
    );

    let update = create_action(move |state: &StateEvent| {
        set_error(None);
        let state = state.to_owned();
        let inner = mr().to_owned();
        async move {
            match gitlab().update_mr(&inner, &state).await {
                Ok(new_mr) => mr.set(new_mr),
                Err(err) => set_error(Some(err)),
            }
        }
    });
    let merge = create_action(move |()| {
        set_error(None);
        let inner = mr().to_owned();
        async move {
            match gitlab().merge_mr(&inner).await {
                Ok(new_mr) => mr.set(new_mr),
                Err(err) => set_error(Some(err)),
            }
        }
    });

    view! {
        <tr class="align-middle">
            <td>{move || mr().title}</td>
            <td>
                <a on:click=|e| e.stop_propagation() target="_blank" href=move || mr().web_url>
                    {move || mr().references.full}
                </a>
            </td>
            <td>{move || mr().status}</td>
            <td>
                {move || match pipeline() {
                    Some(pipelines) => {
                        view! {
                            <ul class="list-unstyled mb-0">
                                {pipelines
                                    .into_iter()
                                    .map(|p| view! { <li>{p}</li> })
                                    .collect_view()}

                            </ul>
                        }
                            .into_view()
                    }
                    None => view! { <p>"Loading..."</p> }.into_view(),
                }}

            </td>
            <td>
                <div
                    class="btn-group btn-group-sm"
                    role="group"
                    aria-label="Actions"
                    on:click=|e| e.stop_propagation()
                >
                    <Show when=move || mr.with(|mr| mr.can_merge())>
                        <button
                            type="button"
                            class="btn btn-success"
                            on:click=move |_| merge.dispatch(())
                        >
                            "merge"
                        </button>
                    </Show>
                    <Show when=move || mr.with(|mr| mr.can_close())>
                        <button
                            type="button"
                            class="btn btn-danger"
                            on:click=move |_| update.dispatch(StateEvent::Close)
                        >
                            "close"
                        </button>
                    </Show>
                    <Show when=move || mr.with(|mr| mr.can_reopen())>
                        <button
                            type="button"
                            class="btn btn-warning"
                            on:click=move |_| update.dispatch(StateEvent::Reopen)
                        >
                            "reopen"
                        </button>
                    </Show>
                </div>
            </td>
        </tr>
        {move || {
            error()
                .map(|err| {
                    view! {
                        <tr>
                            <td colspan="6">
                                <Error err/>
                            </td>
                        </tr>
                    }
                })
        }}
    }
}
