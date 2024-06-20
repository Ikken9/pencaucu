use std::time::{Duration, UNIX_EPOCH};
use chrono::{DateTime, Local, NaiveDateTime};
use leptos::*;
use leptos::logging::{error, log};
use crate::models::match_model::Match;
use crate::services::match_service;


#[component]
pub fn Matches() -> impl IntoView {
    let matches_data = create_resource(
        || (),  // The initial state for the resource
        |_| async {
            log!("Fetching matches...");
            let result = match_service::get_matches().await;
            match result {
                Ok(matches) => {
                    log!("Successfully fetched matches.");
                    Some(matches)
                }
                Err(e) => {
                    error!("Error fetching matches: {:?}", e);
                    None
                }
            }
        },
    );

    let (active_tab, set_active_tab) = create_signal("Pending".to_string());

    view! {
        <Suspense fallback=|| view! { "Loading matches..." }>
            {move || matches_data.get().map(|matches| match matches {
                None => view! { <div>"Error loading matches."</div> },
                Some(matches) => {
                    // Classify matches here based on the current time
                    let (playing, pending, ended) = classify_matches(matches);

                    view! {
                        <div class="p-3">
                            <div class= "font-kanit text-xl font-bold italic text-zinc-300">
                            MATCHES
                            </div>
                            <div class="font-kanit text-sm font-medium text-center text-gray-500 border-b border-b-secondary-gray">
                                <ul class="justify-center justify-evenly flex flex-wrap -mb-px">
                                    <li class="me-1">
                                        <a
                                            href="#"
                                            class=move || format!(
                                                "inline-block p-2 border-b-2 rounded-t-lg {} {}",
                                                if *active_tab.get() == String::from("Playing") {
                                                    "text-zinc-300 border-b-4 border-violet-500"
                                                } else {
                                                    "border-transparent hover:text-gray-600 hover:border-gray-300"
                                                },
                                                if *active_tab.get() == String::from("Playing") { "active" } else { "" }
                                            )
                                            on:click=move |_| set_active_tab(String::from("Playing"))
                                        >
                                            "Playing"
                                        </a>
                                    </li>
                                    <li class="me-1">
                                        <a
                                            href="#"
                                            class=move || format!(
                                                "inline-block p-2 border-b-2 rounded-t-lg {} {}",
                                                if *active_tab.get() == String::from("Pending") {
                                                    "text-zinc-300 border-b-4 border-violet-500"
                                                } else {
                                                    "border-transparent hover:text-gray-600 hover:border-gray-300"
                                                },
                                                if *active_tab.get() == String::from("Pending") { "active" } else { "" }
                                            )
                                            on:click=move |_| set_active_tab(String::from("Pending"))
                                        >
                                            "Pending"
                                        </a>
                                    </li>
                                    <li class="me-1">
                                        <a
                                            href="#"
                                            class=move || format!(
                                                "inline-block p-2 border-b-4 border-b-2 rounded-t-lg {} {}",
                                                if *active_tab.get() == String::from("Ended") {
                                                    "text-zinc-300 border-violet-500"
                                                } else {
                                                    "border-transparent hover:text-gray-600 hover:border-gray-300"
                                                },
                                                if *active_tab.get() == String::from("Ended") { "active" } else { "" }
                                            )
                                            on:click=move |_| set_active_tab(String::from("Ended"))
                                        >
                                            "Ended"
                                        </a>
                                    </li>
                                </ul>
                            </div>
                            <div class="tab-content mt-4">
                                { match &*active_tab.get() {
                                    "Playing" => view! { <MatchList matches=playing.clone() bettable=false /> }.into_view(),
                                    "Pending" => view! { <MatchList matches=pending.clone() bettable=true /> }.into_view(),
                                    "Ended" => view! { <MatchList matches=ended.clone() bettable=false /> }.into_view(),
                                    _ => view! { <div>"Unknown tab"</div> }.into_view(),
                                } }
                            </div>
                        </div>
                    }
                }
            })}
        </Suspense>
    }
}

#[component]
pub fn Match(match_data: Match, bettable: bool) -> impl IntoView {
    let formatted_date = u64_to_date(match_data.date).format("%A %d, %B - %H:%M").to_string();
    view! {
        <div class="match-card bg-gradient-to-r from-primary-gray-1 to-primary-gray-2 p-2 rounded-lg shadow-md flex flex-col items-center mb-1 sm:p-4 h-24">
            <div class="flex items-center justify-between w-full">
                <div class="flex items-center relative z-10">
                    <img class="h-14 w-14 rounded-lg" src={match_data.team_picture_url} alt="first-team-name"/>
                    <div class="ml-2 text-sm sm:ml-3 sm:text-base">
                        <div class="font-semibold text-slate-50">{&match_data.first_team_name}</div>
                    </div>
                </div>
                <Show when=move || {bettable == true} fallback=|| view! { <div></div> }>
                    <button type="button" class="mt-2 absolute left-1/2 transform -translate-x-1/2 z-10 text-white bg-gradient-to-r from-purple-500 via-purple-600 to-purple-700 hover:bg-gradient-to-br focus:ring-4 focus:outline-none focus:ring-purple-300 font-medium rounded-lg text-sm px-4 py-1.5 text-center me-1.5 mb-0.5">
                        Bet
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
pub fn MatchList(matches: Vec<Match>, bettable: bool) -> impl IntoView {
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
                        <Match match_data=match_memo() bettable=bettable />
                    }
                }
            />
        </div>
    }
}

fn classify_matches(matches: Vec<Match>) -> (Vec<Match>, Vec<Match>, Vec<Match>) {
    let now = Local::now().naive_local();

    let mut playing = vec![];
    let mut pending = vec![];
    let mut ended = vec![];

    for match_item in matches {
        let match_date = NaiveDateTime::from_timestamp((match_item.date / 1000) as i64, 0);
        if match_date > now {
            pending.push(match_item);
        } else if now - match_date < chrono::Duration::minutes(90) {
            playing.push(match_item);
        } else {
            ended.push(match_item);
        }
    }

    (playing, pending, ended)
}

fn u64_to_date(timestamp: u64) -> DateTime<Local> {
    // Convert u64 timestamp (milliseconds since UNIX_EPOCH) to SystemTime
    let unix_epoch = UNIX_EPOCH;
    let system_time = unix_epoch + Duration::from_millis(timestamp);

    // Convert SystemTime to DateTime<Local> (or other timezone if needed)
    DateTime::<Local>::from(system_time)
}