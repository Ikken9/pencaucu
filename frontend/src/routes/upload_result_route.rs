use chrono::{Local, NaiveDateTime};
use leptos::*;
use leptos::leptos_dom::{error, log};
use leptos_router::{NavigateOptions, use_navigate, use_params_map};
use crate::models::match_model::Match;
use crate::Navbar;
use crate::services::{bet_service, match_service, result_service};
use crate::services::match_service::timestamp_to_date;

#[component]
pub fn MakeResult() -> impl IntoView {
    let (result_error, set_result_error) = create_signal(None::<String>);
    let (wait_for_response, set_wait_for_response) = create_signal(false);

    let params = use_params_map();

    let match_id = move || {
        params.with(|params| params.get("match-id").cloned())
    };

    if let Some(match_id) = match_id().clone() {
        let window = web_sys::window().expect("No global window exists");
        let local_storage = window.local_storage().expect("").expect("local storage is `None`");
        local_storage.set_item("matchId", &match_id.to_string()).expect("should be able to save match id in the local storage");
    } else {
        error!("Unable to save matchId into local storage")
    }

    let result_action =
        create_action(move |(team_score, faced_team_score): &(String, String)| {
            let team_score = team_score.clone();
            let faced_team_score = faced_team_score.clone();

            async move {
                set_wait_for_response.set(true);

                let res = result_service::submit_result(
                    &match_id().unwrap().parse::<u32>().unwrap(),
                    &team_score.parse::<u8>().unwrap(),
                    &faced_team_score.parse::<u8>().unwrap()
                )
                    .await;

                match res {
                    Ok(_) => {
                        set_result_error.set(None);
                    }
                    Err(e) => {
                        set_result_error.set(Some(e.to_string()));
                    }
                }
                set_wait_for_response.set(false);
            }
        });

    let disabled = Signal::derive(move || wait_for_response.get());

    view! {
        <SubmitResultForm
            action_label="Submit Match Result"
            action=result_action
            error=result_error.into()
            disabled
        />
    }
}

#[component]
pub fn UploadResult() -> impl IntoView {
    let pending_results = create_resource(
        || (),  // The initial state for the resource
        |_| async {
            log!("Fetching pending results...");
            let result = result_service::get_pending_results().await;
            match result {
                Ok(pending_results) => {
                    log!("Successfully fetched pending results.");
                    Some(pending_results)
                }
                Err(e) => {
                    error!("Error fetching pending results: {:?}", e);
                    None
                }
            }
        },
    );

    let submitted_results = create_resource(
        || (),  // The initial state for the resource
        |_| async {
            log!("Fetching submitted results...");
            let result = result_service::get_submitted_results().await;
            match result {
                Ok(submitted_results) => {
                    log!("Successfully fetched submitted results.");
                    Some(submitted_results)
                }
                Err(e) => {
                    error!("Error fetching submitted results: {:?}", e);
                    None
                }
            }
        },
    );

    let (active_tab, set_active_tab) = create_signal("Pending to Submit".to_string());

    view! {
        <Suspense fallback=|| view! { "Loading matches..." }>
            {move || pending_results.get().map(|pending | match pending {
                None => view! { <div>"Error loading matches." </div> },
                Some(pending) => {

                    view! {
                        <div class="p-3">
                            <div class= "font-kanit text-xl font-bold italic text-zinc-300">
                            UPLOAD/EDIT A RESULT
                            </div>
                            <div class="font-kanit text-sm font-medium text-center text-gray-500 border-b border-b-secondary-gray">
                                <ul class="justify-center justify-evenly flex flex-wrap -mb-px">
                                    <li class="me-1">
                                        <a
                                            href="#"
                                            class=move || format!(
                                                "inline-block p-2 border-b-2 rounded-t-lg {} {}",
                                                if *active_tab.get() == String::from("Submitted") {
                                                    "text-zinc-300 border-b-4 border-violet-500"
                                                } else {
                                                    "border-transparent hover:text-gray-600 hover:border-gray-300"
                                                },
                                                if *active_tab.get() == String::from("Submitted") { "active" } else { "" }
                                            )
                                            on:click=move |_| set_active_tab(String::from("Submitted"))
                                        >
                                            "Submitted"
                                        </a>
                                    </li>
                                    <li class="me-1">
                                        <a
                                            href="#"
                                            class=move || format!(
                                                "inline-block p-2 border-b-2 rounded-t-lg {} {}",
                                                if *active_tab.get() == String::from("Pending to Submit") {
                                                    "text-zinc-300 border-b-4 border-violet-500"
                                                } else {
                                                    "border-transparent hover:text-gray-600 hover:border-gray-300"
                                                },
                                                if *active_tab.get() == String::from("Pending to Submit") { "active" } else { "" }
                                            )
                                            on:click=move |_| set_active_tab(String::from("Pending to Submit"))
                                        >
                                            "Pending to Submit"
                                        </a>
                                    </li>
                                </ul>
                            </div>
                            <div class="tab-content mt-4">
                                { match &*active_tab.get() {
                                    "Submitted" => view! { <MatchList matches=submitted_results.get().unwrap().unwrap() submitted=false /> }.into_view(),
                                    "Pending to Submit" => view! { <MatchList matches=pending.clone() submitted=true /> }.into_view(),
                                    _ => view! { <div>"Unknown tab"</div> }.into_view(),
                                } }
                            </div>
                        </div>
                    }
                }})}
        </Suspense>
        <Navbar/>
    }
}

#[component]
pub fn Match(match_data: Match, submitted: bool) -> impl IntoView {
    let formatted_date = timestamp_to_date(match_data.date).format("%A %d, %B - %H:%M").to_string();
    view! {
        <div class="match-card bg-gradient-to-r from-primary-gray-1 to-primary-gray-2 p-2 rounded-lg shadow-md flex flex-col items-center mb-1 sm:p-4 h-24">
            <div class="flex items-center justify-between w-full">
                <div class="flex items-center relative z-10">
                    <img class="h-14 w-14 rounded-lg" src={match_data.team_picture_url} alt="first-team-name"/>
                    <div class="ml-2 text-sm sm:ml-3 sm:text-base">
                        <div class="font-semibold text-slate-50">{&match_data.first_team_name}</div>
                    </div>
                </div>
                <Show when=move || {submitted == false} fallback=move || view! {
                    <button type="button" class="mt-2 absolute left-1/2 transform -translate-x-1/2 z-10 text-white bg-gradient-to-r from-purple-500 via-purple-600 to-purple-700 hover:bg-gradient-to-br focus:ring-4 focus:outline-none focus:ring-purple-300 font-medium rounded-lg text-sm px-4 py-1.5 text-center me-1.5 mb-0.5"
                        on:click=move |_| {
                            let match_id = match_data.id.clone();
                            let navigate = use_navigate().clone();
                            spawn_local(async move {
                                spawn_local(async move {
                                    use_navigate()(&format!("/admin-panel/upload-result/{}", match_id), NavigateOptions::default());
                                });
                            });
                        }
                        >
                        Load
                    </button>}>
                    <button type="button" class="mt-2 absolute left-1/2 transform -translate-x-1/2 z-10 text-white bg-gradient-to-r from-purple-500 via-purple-600 to-purple-700 hover:bg-gradient-to-br focus:ring-4 focus:outline-none focus:ring-purple-300 font-medium rounded-lg text-sm px-4 py-1.5 text-center me-1.5 mb-0.5"
                        on:click=move |_| {
                            let match_id = match_data.id.clone();
                            let stored_email = web_sys::window().unwrap().local_storage().unwrap().unwrap().get_item("playerEmail").unwrap();
                            let navigate = use_navigate().clone();
                            spawn_local(async move {
                                spawn_local(async move {
                                    use_navigate()(&format!("/admin-panel/upload-result/{}", match_id), NavigateOptions::default());
                                });
                            });
                        }
                    >
                        Update
                    </button>
                </Show>
                <div class="flex items-center relative z-10">
                    <div class="mr-2 text-sm sm:mr-4 sm:text-base">
                        <div class="font-semibold text-slate-50">{&match_data.second_team_name}</div>
                    </div>
                    <img class="h-14 w-14 rounded-lg" src={match_data.faced_team_picture_url} alt="second-team-name" />
                </div>
            </div>
            <div class="flex items-center justify-between w-full mt-2 pt-1 text-gray-600 text-xs sm:text-sm border-t border-secondary-gray-2 mb-4">
                <div class="text-zinc-300">{formatted_date}</div>
                <div class="text-zinc-300">{match_data.stadium_name}</div>
            </div>
        </div>
    }
}

#[component]
pub fn MatchList(matches: Vec<Match>, submitted: bool) -> impl IntoView {
    view! {
        <div class="grid gap-2">
            <For
                each=move || matches.clone().into_iter().enumerate()
                key=|(_, match_data)| match_data.id.clone()
                children=move |(_, match_item)| {
                    let match_memo = create_memo(move |_| {
                        match_item.clone()
                    });
                    view! {
                        <Match match_data=match_memo() submitted=submitted />
                    }
                }
            />
        </div>
    }
}

#[component]
pub fn SubmitResultForm(
    action_label: &'static str,
    action: Action<(String, String), ()>,
    error: Signal<Option<String>>,
    disabled: Signal<bool>,
) -> impl IntoView {
    let (team_score, set_team_score) = create_signal(String::new());
    let (faced_team_score, set_faced_team_score) = create_signal(String::new());

    let dispatch_action = move || action.dispatch((team_score.get(), faced_team_score.get()));

    let button_is_disabled = Signal::derive(move || {
        disabled.get() || team_score.get().is_empty() || faced_team_score.get().is_empty()
    });

    let match_data = create_resource(
        || (),  // The initial state for the resource
        move |_| {  // Use the `move` keyword here
            async move {
                match web_sys::window().unwrap().local_storage().unwrap().unwrap().get_item("matchId").unwrap() {
                    Some(match_id) => {
                        match match_service::get_match(match_id.to_string()).await {
                            Ok(m) => {
                                log!("Successfully obtained match.");
                                Some(m)
                            }
                            Err(e) => {
                                error!("Error obtaining match: {:?}", e);
                                None
                            }
                        }
                    }
                    None => {
                        None
                    }
                }
            }
        },
    );

    view! {
        <div class="flex min-h-full flex-col justify-center p-3 ">
            <div class= "font-kanit text-xl font-bold italic text-zinc-300 mb-2">
            PLACE MATCH RESULT
            </div>
            <Suspense fallback=|| view! { "Loading match..." }>
                {move || match_data.get().map(|match_data| match match_data {
                    None => view! { <div>"Error loading match data."</div> },
                    Some(match_data) => view! {
                        <div class="match-card flex flex-col items-center mb-1 sm:p-4 bg-gradient-to-r from-primary-gray-1 to-primary-gray-2 p-2 rounded-lg shadow-md">
                            <div class="flex items-start justify-between w-full">
                                <div class="text-sm sm:text-base max-w-14 text-center">
                                    <img class="h-14 w-14 rounded-lg mb-2 mx-auto" src={match_data.team_picture_url} alt="first-team-flag"/>
                                    <div class="font-semibold text-slate-50">
                                        <p>{match_data.first_team_name}</p>
                                    </div>
                                </div>
                                <div class="flex flex-col items-center justify-center w-full">
                                <div>
                                    <form class="space-y-6" on:submit=move |ev| {
                                        ev.prevent_default();
                                        dispatch_action();
                                    }>
                                        {move || {
                                            error.get().map(|err| {
                                                view! { <p style="color:red;">{err}</p> }
                                            })
                                        }}
                                        <div class="flex relative p-4 h-12">
                                            <div class="float-left mr-8 width-1/2 ">
                                                <input
                                                    id="team-score"
                                                    name="team-score"
                                                    type="text"
                                                    autocomplete="team-score"
                                                    required
                                                    class="w-10 h-10 rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                                                    placeholder=""
                                                    prop:disabled=move || disabled.get()
                                                    on:keyup=move |ev: ev::KeyboardEvent| {
                                                        let val = event_target_value(&ev);
                                                        set_team_score.update(|v| *v = val);
                                                    }
                                                    on:change=move |ev| {
                                                        let val = event_target_value(&ev);
                                                        set_team_score.update(|v| *v = val);
                                                    }
                                                />
                                            </div>
                                            <div class="float-right ml-8 width-1/2">
                                                <input
                                                    id="faced-team-score"
                                                    name="faced-team-score"
                                                    type="text"
                                                    autocomplete="faced-team-score"
                                                    required
                                                    class="w-10 h-10 rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                                                    placeholder=""
                                                    prop:disabled=move || disabled.get()
                                                    on:keyup=move |ev: ev::KeyboardEvent| {
                                                        let val = event_target_value(&ev);
                                                        set_faced_team_score.update(|v| *v = val);
                                                    }
                                                    on:change=move |ev| {
                                                        let val = event_target_value(&ev);
                                                        set_faced_team_score.update(|v| *v = val);
                                                    }
                                                />
                                            </div>
                                        </div>
                                    </form>
                                </div>
                            </div>
                                <div class="text-sm sm:text-base max-w-14 text-center">
                                    <img class="h-14 w-14 rounded-lg mb-2 mx-auto" src={match_data.faced_team_picture_url} alt="second-team-flag" />
                                    <div class="font-semibold text-slate-50">
                                        <p>{match_data.second_team_name}</p>
                                    </div>
                                </div>
                            </div>
                            <div class="flex justify-center w-full mt-4">
                                <button type="submit" class="flex w-full justify-center rounded-md text-white bg-gradient-to-r from-purple-500 via-purple-600 to-purple-700 hover:bg-gradient-to-br focus:ring-4 focus:outline-none focus:ring-purple-300 font-medium rounded-lg text-sm px-4 py-1.5 text-center me-1.5 mb-0.5"
                                prop:disabled=move || button_is_disabled.get()
                                on:click=move |_| dispatch_action()
                                >
                                    {action_label}
                                </button>
                            </div>
                            <div class="flex items-center justify-between w-full mt-4 pt-1 text-gray-600 text-xs sm:text-sm border-t border-secondary-gray-2">
                                <div class="text-zinc-300">{match_data.stadium_name}</div>
                            </div>
                        </div>
                    }
                })}
            </Suspense>
        </div>
    }
}