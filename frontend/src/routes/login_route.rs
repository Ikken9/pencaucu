use leptos::*;
use crate::models::auth_model::{EmailAddress, Password};
use crate::services:: auth_service;


#[component]
pub fn Login() -> impl IntoView {
    let (login_error, _set_login_error) = create_signal(None::<String>);
    let (wait_for_response, set_wait_for_response) = create_signal(false);

    let login_action =
        create_action(move |(email, password): &(String, String)| {
            log::debug!("Try to login with {email}");
            let email = EmailAddress(email.to_string());
            let password = Password(password.to_string());

            async move {
                set_wait_for_response.update(|w| *w = true);
                let _result = auth_service::login(email, password).await;
                set_wait_for_response.update(|w| *w = false);

            }
        });

    let disabled = Signal::derive(move || wait_for_response.get());

    view! {
        <LoginForm
            title="Please login to your account"
            action_label="Login"
            action=login_action
            error=login_error.into()
            disabled
        />
        <p>"Don't have an account?"</p>
        // <A href=Page::Register.path()>"Register"</A>
    }
}

#[component]
fn LoginForm(
    title: &'static str,
    action_label: &'static str,
    action: Action<(String, String), ()>,
    error: Signal<Option<String>>,
    disabled: Signal<bool>,
) -> impl IntoView {
    let (password, set_password) = create_signal(String::new());
    let (email, set_email) = create_signal(String::new());

    let dispatch_action =
        move || action.dispatch((email.get(), password.get()));

    let button_is_disabled = Signal::derive(move || {
        disabled.get() || password.get().is_empty() || email.get().is_empty()
    });

    view! {
        <form on:submit=|ev| ev.prevent_default()>
            <p>{title}</p>
            {move || {
                error
                    .get()
                    .map(|err| {
                        view! { <p style="color:red;">{err}</p> }
                    })
            }}
            <input
                type="email"
                required
                placeholder="Email address"
                prop:disabled=move || disabled.get()
                on:keyup=move |ev: ev::KeyboardEvent| {
                    let val = event_target_value(&ev);
                    set_email.update(|v| *v = val);
                }
                on:change=move |ev| {
                    let val = event_target_value(&ev);
                    set_email.update(|v| *v = val);
                }
            />
            <input
                type="password"
                required
                placeholder="Password"
                prop:disabled=move || disabled.get()
                on:keyup=move |ev: ev::KeyboardEvent| {
                    match &*ev.key() {
                        "Enter" => {
                            dispatch_action();
                        }
                        _ => {
                            let val = event_target_value(&ev);
                            set_password.update(|p| *p = val);
                        }
                    }
                }
                on:change=move |ev| {
                    let val = event_target_value(&ev);
                    set_password.update(|p| *p = val);
                }
            />
            <button
                prop:disabled=move || button_is_disabled.get()
                on:click=move |_| dispatch_action()
            >
                {action_label}
            </button>
        </form>
    }
}