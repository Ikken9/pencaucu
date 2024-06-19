use leptos::*;
use leptos::leptos_dom::{error, log};
use leptos_router::*;
use reqwest::StatusCode;
use crate::models::auth_model::{Username, EmailAddress, Career, Password};
use crate::services::{auth_service, career_service, match_service};

#[component]
pub fn Register() -> impl IntoView {
    let (register_error, set_register_error) = create_signal(None::<String>);
    let (wait_for_response, set_wait_for_response) = create_signal(false);
    let navigate = use_navigate();

    let register_action =
        create_action(move |(username, email, career, password): &(String, String, String, String)| {
            let username = Username(username.to_string());
            let email = EmailAddress(email.to_string());
            let career = Career(career.to_string());
            let password = Password(password.to_string());

            {
                let value = navigate.clone();
                async move {
                    set_wait_for_response.update(|w| *w = true);
                    match auth_service::register(username, email, career, password).await {
                        Ok(res) => {
                            let status_code = res.status();
                            match status_code {
                                StatusCode::CONFLICT => {
                                    set_register_error.set(Some(String::from("That email is already registered")));
                                },
                                StatusCode::CREATED => {
                                    set_register_error.set(None);
                                    value("/login", NavigateOptions::default()); // Navigate to the login page
                                },
                                _ => {}
                            }
                        }
                        Err(e) => {
                            set_register_error.set(Some(e.to_string()));
                        }
                    }
                    set_wait_for_response.update(|w| *w = false);
                }
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
    action: Action<(String, String, String, String), ()>,
    error: Signal<Option<String>>,
    disabled: Signal<bool>,
) -> impl IntoView {
    let (username, set_username) = create_signal(String::new());
    let (email, set_email) = create_signal(String::new());
    let (career, set_career) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());

    let dispatch_action =
        move || action.dispatch((username.get(), email.get(), career.get(), password.get()));

    let button_is_disabled = Signal::derive(move || {
        disabled.get() || username.get().is_empty() || email.get().is_empty() || career.get().is_empty() || password.get().is_empty()
    });

    view! {
        <div class="flex min-h-full flex-col justify-center px-6 py-12 lg:px-8">
            <div class="sm:mx-auto sm:w-full sm:max-w-sm">
                // <img class="mx-auto h-40 w-40" src="https://raw.githubusercontent.com/Ikken9/pencaucu/dev/frontend/assets/logo.png" alt="Copa Logo"/>
                <h2 class="mt-10 text-center text-2xl font-bold leading-9 tracking-tight text-zinc-300">"Create a new account"</h2>
            </div>
            <div class="mt-10 sm:mx-auto sm:w-full sm:max-w-sm p-4">
                <form class="space-y-6" on:submit=move |ev| {
                    ev.prevent_default();
                    dispatch_action();
                }>
                    {move || {
                        error
                            .get()
                            .map(|err| {
                                view! { <p style="color:red;">{err}</p> }
                            })
                    }}
                    <div>
                        <label for="username" class="block text-sm font-medium leading-6 text-zinc-300">"Username"</label>
                        <div class="mt-2">
                             <input id="username" name="username" type="text" autocomplete="username" required class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                                placeholder="Enter your username"
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
                        <label for="email" class="block text-sm font-medium leading-6 text-zinc-300">"Email address"</label>
                        <div class="mt-2">
                            <input id="email" name="email" type="email" autocomplete="email" required class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                                placeholder="example@correo.ucu.edu.uy"
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
                        <button id="dropdownDefaultButton" type="button" data-dropdown-toggle="dropdown" class="flex w-full justify-center font-medium text-sm text-center rounded-md text-white bg-blue-700 hover:bg-blue-800 px-4 py-1.5 me-1.5 mb-0.5"
                        >
                            Choose a career
                            <svg class="w-2.5 h-2.5 ms-4 mt-0.5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 10 1">
                                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m1 1 4 4 4-4"/>
                            </svg>
                        </button>
                        <div id="dropdown" class="z-10 hidden bg-white divide-y divide-gray-100 rounded-lg shadow w-44 dark:bg-gray-700">
                            <Careers/>
                        </div>
                    </div>
                    <div>
                        <label for="password" class="block text-sm font-medium leading-6 text-zinc-300">"Password"</label>
                        <div class="mt-2">
                            <input id="password" name="password" type="password" autocomplete="current-password" required class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                                placeholder="Enter a password"
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
                        <button type="submit" class="flex w-full justify-center rounded-md text-white bg-gradient-to-r from-purple-500 via-purple-600 to-purple-700 hover:bg-gradient-to-br focus:outline-none font-medium rounded-lg text-sm px-4 py-1.5 text-center me-1.5 mb-0.5"
                        prop:disabled=move || button_is_disabled.get()
                        >
                            {action_label}
                        </button>
                    </div>
                        <p class="mt-10 text-center text-sm text-zinc-300">
                            "Already have an account? "
                            <A href="/login" class="font-semibold leading-6 text-indigo-600 hover:text-indigo-500">"Sign In!"</A>
                        </p>
                </form>
            </div>
        </div>
    }
}

#[component]
pub fn Careers() -> impl IntoView {
    let careers_data = create_resource(
        || (),  // The initial state for the resource
        |_| async {
            log!("Fetching careers...");
            let result = career_service::get_careers().await;
            match result {
                Ok(careers) => {
                    log!("Successfully fetched careers.");
                    Some(careers)
                }
                Err(e) => {
                    error!("Error fetching careers: {:?}", e);
                    None
                }
            }
        },
    );

    view! {
        <div class="p-4">
            {move || careers_data.get().map(|careers| match careers {
                None => view! { <div>"Error loading careers."</div> },
                Some(careers_list) => view! {
                    <div>
                        <ul>
                            <For
                                each=move || careers_list.clone().into_iter().enumerate()
                                key=|(_, career)| career.0.clone()
                                children=move |(_, career)| {
                                    view! {
                                        <li href="#" class="block px-4 py-2 hover:bg-gray-100">
                                            {career.0.clone()}
                                        </li>
                                    }
                                }
                            />
                        </ul>
                    </div>
                }
            })}
        </div>
    }
}

