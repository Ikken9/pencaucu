use leptos::*;
use leptos::leptos_dom::{error, log};
use crate::models::bet_model::Bet;
use crate::services::{bet_service, match_service};
use crate::services::match_service::timestamp_to_date;

#[component]
pub fn MakeBet(match_id: u32) -> impl IntoView {
    let (bet_error, set_bet_error) = create_signal(None::<String>);
    let (wait_for_response, set_wait_for_response) = create_signal(false);

    let bet_action =
        create_action(move |(team_score, faced_team_score): &(String, String)| {
            let team_score = team_score.clone();
            let faced_team_score = faced_team_score.clone();

            async move {
                let email = web_sys::window().unwrap().local_storage().unwrap().unwrap().get_item("email").unwrap().unwrap();
                set_wait_for_response.update(|w| *w = true);
                match bet_service::make_bet(&*email, &match_id, &team_score.parse::<u8>().unwrap(), &faced_team_score.parse::<u8>().unwrap()).await {
                    Ok(_) => {
                        set_bet_error.set(None);
                    }
                    Err(e) => {
                        set_bet_error.set(Some(e.to_string()));
                    }
                }
                set_wait_for_response.update(|w| *w = false);
            }
        });

    let disabled = Signal::derive(move || wait_for_response.get());

    view! {
        <BetForm
            action_label="Bet"
            action=bet_action
            error=bet_error.into()
            disabled
        />
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
                Ok(bets) => {
                    log!("Successfully fetched bets.");
                    Some(bets)
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
                            YOUR BETS
                            </div>
                            <div class="font-kanit text-sm text-zinc-300 border-b border-b-secondary-gray mb-2"></div>
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
    let match_id = bet_data.clone().match_id.to_string();
    let match_data = create_resource(
        || (),  // The initial state for the resource
        move |_| {  // Use the `move` keyword here
            let match_id = match_id.clone(); // Move match_id inside the async block
            async move {  // Use the `async move` block
                log!("Fetching match...");
                let result = match_service::get_match(match_id).await;
                match result {
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
                                <img class="h-14 w-14 rounded-lg" src={match_data.team_picture_url} alt="first-team-flag"/>
                                <div class="ml-2 text-sm sm:ml-3 sm:text-base">
                                    <div class="font-semibold text-slate-50">{&match_data.first_team_name}</div>
                                </div>
                            </div>
                            <div class="font-kanit font-semi-bold text-4xl text-zinc-300">{bet_data.team_score} : {bet_data.faced_team_score}</div>
                            <div class="flex items-center relative z-10">
                                <div class="mr-2 text-sm sm:mr-4 sm:text-base">
                                    <div class="font-semibold text-slate-50">{&match_data.second_team_name}</div>
                                </div>
                                <img class="h-14 w-14 rounded-lg" src={match_data.faced_team_picture_url} alt="second-team-flag" />
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

#[component]
pub fn BetForm(
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

    view! {
        <div class="flex min-h-full flex-col justify-center px-6 py-12 lg:px-8">
            <div class="sm:mx-auto sm:w-full sm:max-w-sm">
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
                        <label for="team-score" class="block text-sm font-medium leading-6 text-zinc-300">"Username"</label>
                        <div class="mt-2">
                             <input id="team-score" name="team-score" type="text" autocomplete="team-score" required class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
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
                    </div>
                    <div>
                        <label for="faced-team-score" class="block text-sm font-medium leading-6 text-zinc-300">"Email address"</label>
                        <div class="mt-2">
                            <input id="faced-team-score" name="faced-team-score" type="text" autocomplete="faced-team-score" required class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
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
                    <div>
                        <button type="submit" class="flex w-full justify-center rounded-md text-white bg-gradient-to-r from-purple-500 via-purple-600 to-purple-700 hover:bg-gradient-to-br focus:outline-none font-medium rounded-lg text-sm px-4 py-1.5 text-center me-1.5 mb-0.5"
                        prop:disabled=move || button_is_disabled.get()
                        >
                            {action_label}
                        </button>
                    </div>
                </form>
            </div>
        </div>
    }
}


