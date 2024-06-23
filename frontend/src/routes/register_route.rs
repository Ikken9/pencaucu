use leptos::*;
use leptos::leptos_dom::{error, log};
use leptos_router::*;
use reqwest::header::AUTHORIZATION;
use reqwest::StatusCode;
use crate::models::auth_model::{Username, EmailAddress, Password};
use crate::models::career_model::Career;
use crate::services::{register_service, career_service};

#[component]
pub fn Register() -> impl IntoView {
    let (register_error, set_register_error) = create_signal(None::<String>);
    let (wait_for_response, set_wait_for_response) = create_signal(false);
    let navigate = use_navigate();

    let register_action =
        create_action(move |(username, email, career, password): &(String, String, String, String)| {
            let username = Username(username.to_string());
            let email = EmailAddress(email.to_string());
            let career = Career { name: career.clone() };
            let password = Password(password.to_string());

            let email_clone = email.clone();

            let value = navigate.clone();
            async move {
                set_wait_for_response.update(|w| *w = true);
                match register_service::register(username, email, career, password).await {
                    Ok(res) => {
                        let status_code = res.status();
                        match status_code {
                            StatusCode::CONFLICT => {
                                set_register_error.set(Some(String::from("That email is already registered")));
                            },
                            StatusCode::CREATED => {
                                let window = web_sys::window().expect("No global window exists");
                                let local_storage = window.local_storage().expect("").expect("local storage is `None`");
                                local_storage.set_item("email", &email_clone.to_string()).expect("should be able to save player email in the local storage");

                                if let Some(auth_header) = res.headers().get(AUTHORIZATION) {
                                    if let Ok(token) = auth_header.to_str() {
                                        local_storage.set_item("token", token).expect("should be able to set item in local storage");
                                    }
                                } else {
                                    log!("Authorization header not found");
                                }
                                set_register_error.set(None);
                                value("/login", NavigateOptions::default()); // Navigate to the login page
                            },
                            StatusCode::INTERNAL_SERVER_ERROR => {
                                set_register_error.set(Some(String::from("Something went wrong...")));
                            }
                            _ => {}
                        }
                    }
                    Err(e) => {
                        set_register_error.set(Some(e.to_string()));
                    }
                }
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
    action: Action<(String, String, String, String), ()>,
    error: Signal<Option<String>>,
    disabled: Signal<bool>,
) -> impl IntoView {
    let (username, set_username) = create_signal(String::new());
    let (email, set_email) = create_signal(String::new());
    let (career, set_career) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());

    // Fetch careers data
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

    let dispatch_action = move || action.dispatch((username.get(), email.get(), career.get(), password.get()));

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
                        error.get().map(|err| {
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
                        <label for="career" class="block text-sm font-medium leading-6 text-zinc-300">"Career"</label>
                        <div class="mt-2">
                            <select id="career" name="career" required class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                                prop:disabled=move || disabled.get()
                                on:change=move |ev| {
                                    let val = event_target_value(&ev);
                                    set_career.update(|v| *v = val);
                                }
                            >
                                <option value="">Select a career</option>
                                {move || careers_data.get().map(|careers| {
                                    if let Some(careers_list) = careers {
                                        careers_list.iter().map(|c| {
                                            view! { <option value={c.name.clone()}>{c.name.clone()}</option> }
                                        }).collect::<Vec<_>>()
                                    } else {
                                        vec![view! { <option value="">Error loading careers</option> }]
                                    }
                                }).unwrap_or_else(|| vec![view! { <option value="">Error loading careers</option> }])}
                            </select>
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
