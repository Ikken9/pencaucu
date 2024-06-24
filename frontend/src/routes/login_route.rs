use leptos::*;
use leptos::logging::log;
use leptos_router::*;
use reqwest::header::AUTHORIZATION;
use reqwest::StatusCode;
use crate::models::auth_model::{EmailAddress, Password};
use crate::services::login_service;


#[component]
pub fn Login() -> impl IntoView {
    let (login_error, set_login_error) = create_signal(None::<String>);
    let (wait_for_response, set_wait_for_response) = create_signal(false);
    let navigate = use_navigate();

    let login_action = create_action(move |(email, password): &(String, String)| {
        let email = EmailAddress(email.clone());
        let password = Password(password.clone());

        let email_clone = email.clone();

        let navigate = navigate.clone();
        async move {
            set_wait_for_response.set(true);
            match login_service::login(email, password).await {
                Ok(res) => {
                    let status_code = res.status();
                    match status_code {
                        StatusCode::BAD_REQUEST => {
                            set_login_error.set(Some("Unable to login".to_string()));
                        },
                        StatusCode::OK | StatusCode::NOT_MODIFIED => {
                            let window = web_sys::window().expect("No global window exists");
                            let local_storage = window.local_storage().unwrap().expect("local storage is `None`");
                            local_storage.set_item("email", &email_clone.to_string()).expect("should be able to save player email in the local storage");

                            if let Some(auth_header) = res.headers().get(AUTHORIZATION) {
                                if let Ok(token) = auth_header.to_str() {
                                    local_storage.set_item("token", token).expect("should be able to set item in local storage");
                                }
                            } else {
                                log!("Authorization header not found");
                            }
                            set_login_error.set(None);
                            navigate("/matches", NavigateOptions::default());
                        },
                        StatusCode::INTERNAL_SERVER_ERROR => {
                            set_login_error.set(Some("Something went wrong...".to_string()));
                        }
                        _ => {
                            set_login_error.set(Some("Unexpected error occurred".to_string()));
                        }
                    }
                }
                Err(e) => {
                    set_login_error.set(Some(e.to_string()));
                }
            }
            set_wait_for_response.set(false);
        }
    });

    let disabled = Signal::derive(move || wait_for_response.get());

    view! {
        <LoginForm
            action_label="Sign In"
            action=login_action
            error=login_error.into()
            disabled
        />
    }
}

#[component]
fn LoginForm(
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
        <div class="flex min-h-full flex-col justify-center px-6 py-12 lg:px-8">
            <div class="sm:mx-auto sm:w-full sm:max-w-sm">
                // <img class="mx-auto h-40 w-40" src="https://raw.githubusercontent.com/Ikken9/pencaucu/dev/frontend/assets/logo.png" alt="Copa Logo"/>
                <h2 class="mt-10 text-center text-2xl font-bold leading-9 tracking-tight text-zinc-300">Sign in to your account</h2>
            </div>
            <div class="mt-10 sm:mx-auto sm:w-full sm:max-w-sm p-4 sm:p-8">
                <form class="space-y-6" on:submit=|ev| ev.prevent_default()>
                    {move || {
                        error
                            .get()
                            .map(|err| {
                                view! { <p style="color:red;">{err}</p> }
                            })
                    }}
                    <div>
                        <label for="email" class="block text-sm font-medium leading-6 text-zinc-300">Email address</label>
                        <div class="mt-2">
                            <input id="email" name="email" type="email" autocomplete="email" required class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 ring-gradient-to-r from-purple-500 via-purple-600 to-purple-700 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-purple-600 sm:text-sm sm:leading-6"
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
                        </div>
                    </div>
                    <div>
                        <div class="flex items-center justify-between">
                            <label for="password" class="block text-sm font-medium leading-6 text-zinc-300">Password</label>
                        </div>
                        <div class="mt-2">
                            <input id="password" name="password" type="password" autocomplete="current-password" required class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 ring-gradient-to-r from-purple-500 via-purple-600 to-purple-700 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-purple-600 sm:text-sm sm:leading-6"
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
                        </div>
                    </div>
                    <div>
                        <button type="submit" class="flex w-full justify-center rounded-md text-white bg-gradient-to-r from-purple-500 via-purple-600 to-purple-700 hover:bg-gradient-to-br focus:ring-4 focus:outline-none focus:ring-purple-300 font-medium rounded-lg text-sm px-4 py-1.5 text-center me-1.5 mb-0.5"
                        prop:disabled=move || button_is_disabled.get()
                        on:click=move |_| dispatch_action()
                        >
                            {action_label}
                        </button>
                    </div>
                    <p class="mt-10 text-center text-sm text-zinc-300">
                        "Don't have an account? "
                        <A href="/" class="font-semibold leading-6 text-indigo-600 hover:text-indigo-500">Sign Up!</A>
                    </p>
                </form>
            </div>
        </div>
    }
}