use std::collections::HashSet;

use leptos::*;

use crate::{
    components::Error,
    gitlab::{Gitlab, MergeRequest, StateEvent, ID},
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
    let selected_ids = create_rw_signal(HashSet::new());
    let all_ids = merge_requests
        .iter()
        .map(|mr| mr.with(|mr| mr.id))
        .collect::<HashSet<_>>();

    let num_mrs = merge_requests.len();
    let selected = create_memo(move |_| match selected_ids().len() {
        0 => CheckboxState::Unchecked,
        n if n == num_mrs => CheckboxState::Checked,
        _ => CheckboxState::Indeterminate,
    });

    let set_all = move |v: bool| {
        if v {
            selected_ids.set(all_ids.clone());
        } else {
            selected_ids.set(HashSet::new());
        }
    };

    view! {
        <table class="table">
            <thead>
                <tr>
                    <th scope="col">
                        <input
                            type="checkbox"
                            on:input=move |e| set_all(event_target_checked(&e))
                            prop:checked=move || selected() == CheckboxState::Checked
                            prop:indeterminate=move || selected() == CheckboxState::Indeterminate
                        />
                    </th>
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
                        view! { <MergeRequest mr selected_ids/> }
                    })
                    .collect_view()}
            </tbody>
        </table>
    }
}

#[component]
pub fn MergeRequest(
    mr: RwSignal<MergeRequest>,
    selected_ids: RwSignal<HashSet<ID>>,
) -> impl IntoView {
    let gitlab = use_context::<Signal<Gitlab>>().expect("Gitlab client provided");
    let id = mr.with_untracked(|mr| mr.id);
    let (error, set_error) = create_signal(None);

    let toggle_id = move || {
        selected_ids.update(|ids| {
            if ids.contains(&id) {
                ids.remove(&id);
            } else {
                ids.insert(id);
            }
        });
    };

    let selected = create_memo(move |_| selected_ids.with(|ids| ids.contains(&id)));

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
        <tr
            role="button"
            class="align-middle"
            class:table-active=selected
            on:click=move |_| toggle_id()
        >
            <td>
                <input type="checkbox" prop:checked=selected/>
            </td>
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
                        pipelines.into_iter().map(|p| p.status.into_view()).collect_view()
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
