use leptos::*;
use leptos::leptos_dom::{error, log};
use leptos_router::{NavigateOptions, use_navigate, use_params, use_params_map};
use crate::models::bet_model::Bet;
use crate::Navbar;
use crate::services::{bet_service, match_service};
use crate::services::match_service::timestamp_to_date;

#[component]
pub fn MakeBet() -> impl IntoView {
    let (bet_error, set_bet_error) = create_signal(None::<String>);
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

    let bet_action =
        create_action(move |(team_score, faced_team_score): &(String, String)| {
            let team_score = team_score.clone();
            let faced_team_score = faced_team_score.clone();

            async move {
                let email = web_sys::window().unwrap().local_storage().unwrap().unwrap().get_item("email").unwrap().unwrap();
                set_wait_for_response.update(|w| *w = true);

                let res = bet_service::make_bet(
                    &*email,
                    &match_id().unwrap().parse::<u32>().unwrap(),
                    &team_score.parse::<u8>().unwrap(),
                    &faced_team_score.parse::<u8>().unwrap()
                )
                    .await;

                match res {
                    Ok(_) => {
                        set_bet_error.set(None);
                        use_navigate()("/bets", NavigateOptions::default())
                    }
                    Err(e) => {
                        set_bet_error.set(Some(e.to_string()));
                    }
                }
                set_wait_for_response.update(|w| *w = false);
            }
        });

    let update_bet_action =
        create_action(move |(team_score, faced_team_score): &(String, String)| {
            let team_score = team_score.clone();
            let faced_team_score = faced_team_score.clone();

            async move {
                let email = web_sys::window().unwrap().local_storage().unwrap().unwrap().get_item("email").unwrap().unwrap();
                set_wait_for_response.update(|w| *w = true);

                let res = bet_service::edit_bet(
                    &*email,
                    &match_id().unwrap().parse::<u32>().unwrap(),
                    &team_score.parse::<u8>().unwrap(),
                    &faced_team_score.parse::<u8>().unwrap()
                )
                    .await;

                match res {
                    Ok(_) => {
                        set_bet_error.set(None);
                        use_navigate()("/bets", NavigateOptions::default())
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
            update_action=update_bet_action
            error=bet_error.into()
            disabled
        />
        <Navbar/>
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
        <Navbar/>
    }
}

#[component]
pub fn Bet(bet_data: Bet) -> impl IntoView {
    let match_id = bet_data.match_id.to_string();
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
    update_action: Action<(String, String), ()>,
    error: Signal<Option<String>>,
    disabled: Signal<bool>,
) -> impl IntoView {
    let (team_score, set_team_score) = create_signal(String::new());
    let (faced_team_score, set_faced_team_score) = create_signal(String::new());

    let (updating, set_updating) = create_signal(false);

    let dispatch_action = move || action.dispatch((team_score.get(), faced_team_score.get()));

    let dispatch_update_action = move || update_action.dispatch((team_score.get(), faced_team_score.get()));

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

    let bet_data = create_resource(
        || (),
        move |_| {
            async move {
                match web_sys::window().unwrap().local_storage().unwrap().unwrap().get_item("matchId").unwrap() {
                    Some(match_id) => {
                        let email = web_sys::window().unwrap().local_storage().unwrap().unwrap().get_item("email").unwrap().unwrap();
                        match bet_service::get_bets_by_player_by_match_id(email, match_id).await {
                            Ok(bet) => {
                                log!("Successfully obtained player bet.");
                                set_updating.set(true);
                                set_team_score.set(bet.team_score.to_string());
                                set_faced_team_score.set(bet.faced_team_score.to_string());
                                Some(bet)
                            }
                            Err(e) => {
                                error!("Error obtaining bet: {:?}", e);
                                None
                            }
                        }

                    }
                    None => {
                        None
                    }
                }
            }
        }
    );

    view! {
        <div class="flex min-h-full flex-col justify-center p-3 ">
            <div class= "font-kanit text-xl font-bold italic text-zinc-300 mb-2">
            PLACE YOUR BET
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
                                                    value=move || format!(
                                                        "{}", team_score.get()
                                                    )
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
                                                    value=move || format!(
                                                        "{}", faced_team_score()
                                                    )
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
                                <Show when=move || {!updating.get()} fallback=move || view! {
                                    <button
                                        type="submit"
                                        class="w-full mt-4 rounded-md text-white bg-gradient-to-r from-purple-500 via-purple-600 to-purple-700 hover:bg-gradient-to-br focus:outline-none font-medium rounded-lg text-sm px-4 py-1.5 text-center"
                                        prop:disabled=move || button_is_disabled.get()
                                        on:click=move |_| dispatch_update_action()
                                    >
                                        {action_label}
                                    </button>}>
                                    <button
                                        type="submit"
                                        class="w-full mt-4 rounded-md text-white bg-gradient-to-r from-purple-500 via-purple-600 to-purple-700 hover:bg-gradient-to-br focus:outline-none font-medium rounded-lg text-sm px-4 py-1.5 text-center"
                                        prop:disabled=move || button_is_disabled.get()
                                        on:click=move |_| dispatch_action()
                                    >
                                        {action_label}
                                    </button>
                                </Show>
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
