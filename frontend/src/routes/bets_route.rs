use leptos::*;
use leptos::leptos_dom::{error, log};
use leptos_router::*;
use crate::models::bet_model::Bet;
use crate::services::{bet_service, match_service};
use crate::services::match_service::timestamp_to_date;

#[component]
pub fn MakeBet() -> impl IntoView {
    view! {

    }
}

#[component]
pub fn Bets() -> impl IntoView {
    let bets_data = create_resource(
        || (),  // The initial state for the resource
        |_| async {
            log!("Fetching bets...");
            let email = web_sys::window().unwrap().local_storage().unwrap().unwrap().get_item("email").unwrap();
            let result = bet_service::get_bets_by_player(&email.unwrap()).await;
            match result {
                Ok(matches) => {
                    log!("Successfully fetched bets.");
                    Some(matches)
                }
                Err(e) => {
                    error!("Error fetching bets: {:?}", e);
                    None
                }
            }
        },
    );

    view! {
        <Suspense fallback=|| view! { "Loading bets..." }>
            {move || bets_data.get().map(|bets| match bets {
                None => view! { <div>"Error loading bets."</div> },
                Some(bets) => {
                    view! {
                        <div class="p-3">
                            <div class= "font-kanit text-xl font-bold italic text-zinc-300">
                            BETS
                            </div>

                            <div class="grid gap-2">
                                <For
                                    each=move || bets.clone().into_iter().enumerate()
                                    key=|(_, bet_data)| bet_data.match_id.clone()
                                    children=move |(_, bet_item)| {
                                        let bet_memo = create_memo(move |_| {
                                            bet_item.clone()
                                        });
                                        view! {
                                            <Bet bet_data=bet_memo()/>
                                        }
                                    }
                                />
                            </div>
                        </div>
                    }
                }
            })}
        </Suspense>
    }
}

#[component]
pub fn Bet(bet_data: Bet) -> impl IntoView {
    let params = use_params_map();
    let match_data = create_resource(
        move || params.get().get("match-id").cloned().unwrap_or_default(),
        move |id| async move {
            if id.is_empty() {
                None
            } else {
                match_service::get_match(&id).await.ok()
            }
        },
    );

    view! {
        <Suspense fallback=|| view! { "Loading match..." }>
            {move || match_data.get().map(|match_data| match match_data {
                None => view! { <div>"Error loading match data."</div> },
                Some(match_data) => view! {
                    <div class="match-card bg-gradient-to-r from-primary-gray-1 to-primary-gray-2 p-2 rounded-lg shadow-md flex flex-col items-center mb-1 sm:p-4 h-24">
                        <div class="flex items-center justify-between w-full">
                            <div class="flex items-center relative z-10">
                                <img class="h-14 w-14 rounded-lg" src={match_data.team_picture_url} alt="first-team-name"/>
                                <div class="ml-2 text-sm sm:ml-3 sm:text-base">
                                    <div class="font-semibold text-slate-50">{&match_data.first_team_name}</div>
                                </div>
                                <div>{bet_data.team_score}</div>
                            </div>
                            <div class="text-xl text-zinc-300"><p>VS</p></div>
                            <div class="flex items-center relative z-10">
                                <div>{bet_data.faced_team_score}</div>
                                <div class="mr-2 text-sm sm:mr-4 sm:text-base">
                                    <div class="font-semibold text-slate-50">{&match_data.second_team_name}</div>
                                </div>
                                <img class="h-14 w-14 rounded-lg" src={match_data.faced_team_picture_url} alt="second-team-name" />
                            </div>
                        </div>
                        <div class="flex items-center justify-between w-full mt-2 pt-1 text-gray-600 text-xs sm:text-sm border-t border-secondary-gray-2 mb-4">
                            <div class="text-zinc-300">{timestamp_to_date(match_data.date).format("%A %d, %B - %H:%M").to_string();}</div>
                            <div class="text-zinc-300">{match_data.stadium_name}</div>
                        </div>
                    </div>
                }
            })}
        </Suspense>
    }
}


