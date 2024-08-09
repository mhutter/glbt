use std::fmt;

use leptos::*;

#[derive(Clone, Copy, Default)]
pub enum Class {
    #[default]
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
    Light,
    Dark,
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Class::Primary => "primary",
            Class::Secondary => "secondary",
            Class::Success => "success",
            Class::Danger => "danger",
            Class::Warning => "warning",
            Class::Info => "info",
            Class::Light => "light",
            Class::Dark => "dark",
        }
        .fmt(f)
    }
}

/// Render a Bootstrap Alert with the given class.
///
/// See: https://getbootstrap.com/docs/5.3/components/alerts/
#[component]
pub fn Alert(
    class: Class,
    #[prop(into)] title: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=format!("alert alert-{class}") role="alert">
            {title.map(|t| view! { <h4>{t}</h4> })}
            {children()}
        </div>
    }
}

/// Render a Bootstrap Badge with the given class.
///
/// See: https://getbootstrap.com/docs/5.3/components/badge/
#[component]
pub fn Badge(class: Class, label: &'static str) -> impl IntoView {
    view! { <span class=format!("badge text-bg-{class}")>{label}</span> }
}

/// Render the given error in an alert box
#[component]
pub fn Error<E: ToString + 'static>(err: E) -> impl IntoView {
    view! {
        <Alert class=Class::Danger title="Error">
            <p>{err.to_string()}</p>
        </Alert>
    }
}
