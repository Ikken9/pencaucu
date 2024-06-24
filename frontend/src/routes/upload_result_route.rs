use chrono::{Local, NaiveDateTime};
use leptos::*;
use leptos::leptos_dom::{error, log};
use crate::models::match_model::Match;
use crate::Navbar;
use crate::services::{match_service, result_service};
use crate::services::match_service::timestamp_to_date;

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
                            let stored_email = web_sys::window().unwrap().local_storage().unwrap().unwrap().get_item("playerEmail").unwrap();
                            //let navigate = use_navigate();
                            spawn_local(async move {
                                // match stored_email {
                                //     Some(email) => {
                                //         navigate.clone()(&format!("/bets/{}/{}", email, match_id), NavigateOptions::default());
                                //     }
                                //     None => {
                                //         error!("Unable to retrieve player email");
                                //     }
                                // }
                                });
                            }
                        >
                        Load
                    </button>}>
                    <button type="button" class="mt-2 absolute left-1/2 transform -translate-x-1/2 z-10 text-white bg-gradient-to-r from-purple-500 via-purple-600 to-purple-700 hover:bg-gradient-to-br focus:ring-4 focus:outline-none focus:ring-purple-300 font-medium rounded-lg text-sm px-4 py-1.5 text-center me-1.5 mb-0.5"
                        on:click=move |_| {
                            let match_id = match_data.id.clone();
                            let stored_email = web_sys::window().unwrap().local_storage().unwrap().unwrap().get_item("playerEmail").unwrap();
                            //let navigate = use_navigate();
                            spawn_local(async move {
                                // match stored_email {
                                //     Some(email) => {
                                //         navigate.clone()(&format!("/bets/{}/{}", email, match_id), NavigateOptions::default());
                                //     }
                                //     None => {
                                //         error!("Unable to retrieve player email");
                                //     }
                                // }
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