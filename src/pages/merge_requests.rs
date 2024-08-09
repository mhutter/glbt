use std::collections::HashSet;

use leptos::*;

use crate::gitlab::{Gitlab, MergeRequest, ID};

#[component]
pub fn MergeRequests() -> impl IntoView {
    let gitlab = use_context::<Signal<Gitlab>>().expect("Gitlab client provided");

    let merge_requests = create_local_resource(
        || (),
        move |()| async move { gitlab().get_open_mrs().await },
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
pub fn MergeRequestTable(merge_requests: Vec<MergeRequest>) -> impl IntoView {
    let selected_ids = create_rw_signal(HashSet::new());
    let all_ids = merge_requests
        .iter()
        .map(|mr| mr.id)
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
pub fn MergeRequest(mr: MergeRequest, selected_ids: RwSignal<HashSet<ID>>) -> impl IntoView {
    let toggle_id = move || {
        let id = mr.id;
        selected_ids.update(|ids| {
            if ids.contains(&id) {
                ids.remove(&id);
            } else {
                ids.insert(id);
            }
        });
    };

    let selected = create_memo(move |_| selected_ids.with(|ids| ids.contains(&mr.id)));

    view! {
        <tr class:table-active=selected on:click=move |_| toggle_id()>
            <td>
                <input type="checkbox" prop:checked=selected/>
            </td>
            <td>{mr.title}</td>
            <td>
                <a target="_blank" href=mr.web_url>
                    {mr.references.full}
                </a>
            </td>
            <td>"TODO"</td>
            <td>"TODO"</td>
        </tr>
    }
}

#[component]
pub fn Error<E: ToString>(err: E) -> impl IntoView {
    view! {
        <div class="alert alert-danger" role="alert">
            <h4>"Error"</h4>
            <p>{err.to_string()}</p>
        </div>
    }
}
