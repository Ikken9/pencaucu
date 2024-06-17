use leptos::*;
use crate::models::auth_model::{Username, EmailAddress, Password};
use crate::services::auth_service;

#[component]
pub fn Register() -> impl IntoView {
    let (register_error, _set_register_error) = create_signal(None::<String>);
    let (wait_for_response, set_wait_for_response) = create_signal(false);

    let register_action =
        create_action(move |(username, email, password): &(String, String, String)| {
            let username = Username(username.to_string());
            let email = EmailAddress(email.to_string());
            let password = Password(password.to_string());

            async move {
                set_wait_for_response.update(|w| *w = true);
                let _result = auth_service::register(username, email, password).await;
                set_wait_for_response.update(|w| *w = false);
            }
        });

    let disabled = Signal::derive(move || wait_for_response.get());

    view! {
        <RegisterForm
            action_label="Create account"
            action=register_action
            error=register_error.into()
            disabled
        />
    }
}

#[component]
fn RegisterForm(
    action_label: &'static str,
    action: Action<(String, String, String), ()>,
    error: Signal<Option<String>>,
    disabled: Signal<bool>,
) -> impl IntoView {
    let (username, set_username) = create_signal(String::new());
    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());


    let dispatch_action =
        move || action.dispatch((username.get(), email.get(), password.get()));

    let button_is_disabled = Signal::derive(move || {
        disabled.get() || username.get().is_empty() || email.get().is_empty() || password.get().is_empty()
    });

    view! {
        <div class="flex min-h-full flex-col justify-center px-6 py-12 lg:px-8">
            <div class="sm:mx-auto sm:w-full sm:max-w-sm">
                <img class="mx-auto h-40 w-40" src="https://raw.githubusercontent.com/Ikken9/pencaucu/dev/frontend/assets/logo.png" alt="Copa Logo"/>
                <h2 class="mt-10 text-center text-2xl font-bold leading-9 tracking-tight text-gray-900">Create a new account</h2>
            </div>
            <div class="mt-10 sm:mx-auto sm:w-full sm:max-w-sm p-4 bg-white border border-gray-200 rounded-lg shadow sm:p-8 dark:bg-gray-800 dark:border-gray-700">
                <form class="space-y-6" on:submit=|ev| ev.prevent_default()>
                    {move || {
                        error
                            .get()
                            .map(|err| {
                                view! { <p style="color:red;">{err}</p> }
                            })
                    }}
                    <div>
                        <label for="username" class="block text-sm font-medium leading-6 text-gray-900">Username</label>
                        <div class="mt-2">
                             <input id="username" name="username" type="email" autocomplete="username" required class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                                placeholder="Username"
                                prop:disabled=move || disabled.get()
                                on:keyup=move |ev: ev::KeyboardEvent| {
                                    let val = event_target_value(&ev);
                                    set_username.update(|v| *v = val);
                                }
                                on:change=move |ev| {
                                    let val = event_target_value(&ev);
                                    set_username.update(|v| *v = val);
                                }
                            />
                        </div>
                    </div>
                    <div>
                        <label for="email" class="block text-sm font-medium leading-6 text-gray-900">Email address</label>
                        <div class="mt-2">
                            <input id="email" name="email" type="email" autocomplete="email" required class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
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
                        <label for="password" class="block text-sm font-medium leading-6 text-gray-900">Password</label>
                        <div class="mt-2">
                            <input id="password" name="password" type="password" autocomplete="current-password" required class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
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
                        <button type="submit" class="flex w-full justify-center rounded-md bg-indigo-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                        prop:disabled=move || button_is_disabled.get()
                        on:click=move |_| dispatch_action()
                        >
                            {action_label}
                        </button>
                    </div>
                </form>
            </div>
        </div>
    }
}