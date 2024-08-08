use leptos::*;

use crate::gitlab::Gitlab;

#[component]
pub fn Login(set_gitlab: WriteSignal<Option<Gitlab>>) -> impl IntoView {
    let (url, set_url) = create_signal(String::new());
    let (token, set_token) = create_signal(String::new());
    let (username, set_username) = create_signal(None);
    let (error, set_error) = create_signal(None);

    create_effect(move |_| {
        let _ = url();
        let _ = token();
        set_username(None);
        set_error(None);
    });

    let test_credentials = create_action(move |creds: &(String, String)| {
        let (url, token) = creds.to_owned();
        async move {
            match test_credentials(&url, &token).await {
                Ok(username) => {
                    set_error(None);
                    set_username(Some(username));
                }
                Err(err) => {
                    set_username(None);
                    set_error(Some(err));
                }
            }
        }
    });

    let connect = move || {
        let url = url();
        let token = token();
        let gitlab = Gitlab::new(&url, &token).expect("valid credentials at that point");

        set_gitlab(Some(gitlab));
    };

    // on_cleanup(move || set_gitlab.dispose()); // TODO: ???

    view! {
        <div class="container">
            <h1>"Login"</h1>
            <div class="row mb-3">
                <label for="url" class="col-2 col-form-label">
                    "GitLab URL"
                </label>
                <div class="col-10">
                    <input
                        type="url"
                        id="url"
                        class="form-control"
                        required
                        placeholder="https://gitlab.example.com"
                        prop:value=url
                        on:input=move |e| set_url(event_target_value(&e))
                    />
                    <div class="form-text">
                        "Server base URL, without the leading `api/v4` part"
                    </div>
                </div>
            </div>

            <div class="row mb-3">
                <label for="token" class="col-2 col-form-label">
                    "GitLab Access Token"
                </label>
                <div class="col-10">
                    <input
                        type="password"
                        id="token"
                        class="form-control"
                        required
                        placeholder="glpat-..."
                        prop:value=token
                        on:input=move |e| set_token(event_target_value(&e))
                    />
                    <div class="form-text">
                        "Requires the `api` scope. "
                        {move || match url().as_str() {
                            "" => None,
                            url => {
                                let url = format!(
                                    "{url}/-/user_settings/personal_access_tokens?scopes=api",
                                );
                                Some(
                                    view! {
                                        "Get from "
                                        <a target="_blank" href=&url>
                                            {url}
                                        </a>
                                    },
                                )
                            }
                        }}

                    </div>
                </div>
            </div>

            <div class="row mb-3">
                <div class="offset-2 col-10">
                    {move || match username() {
                        Some(username) => {
                            view! {
                                <button
                                    type="button"
                                    class="btn btn-success"
                                    on:click=move |_| connect()
                                >
                                    "Connect as "
                                    {username}
                                </button>
                            }
                                .into_view()
                        }
                        None => {
                            view! {
                                <button
                                    type="button"
                                    class="btn btn-primary"
                                    on:click=move |_| test_credentials.dispatch((url(), token()))
                                >
                                    "Test credentials"
                                </button>
                                <span class="ms-3 text-danger">{error()}</span>
                            }
                                .into_view()
                        }
                    }}

                </div>
            </div>
        </div>
    }
}

async fn test_credentials(url: &str, token: &str) -> Result<String, String> {
    let gitlab = Gitlab::new(url, token).map_err(|err| err.to_string())?;
    let user = gitlab.get_self().await.map_err(|err| err.to_string())?;
    Ok(user.username)
}
