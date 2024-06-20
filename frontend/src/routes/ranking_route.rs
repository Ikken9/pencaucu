use leptos::*;
use leptos::leptos_dom::{error, log};
use crate::models::player_model::Player;
use crate::services::player_service;

#[component]
pub fn Ranking() -> impl IntoView {
    let players_data = create_resource(
        || (),  // The initial state for the resource
        |_| async {
            log!("Fetching players...");
            let result = player_service::get_players().await;
            match result {
                Ok(players) => {
                    log!("Successfully fetched players.");
                    Some(players)
                }
                Err(e) => {
                    error!("Error fetching players: {:?}", e);
                    None
                }
            }
        },
    );

    view! {
        <Suspense fallback=|| view! { <p class="text-center text-gray-500">"Loading players..."</p> }>
            {move || players_data.get().map(|players| match players {
                None => view! {
                    <div>
                        <p class="text-center text-red-500">"Error loading players."</p>
                    </div>
                },
                Some(players) => view! {
                    <div class="players-container grid gap-2 p-3">
                        <div class="font-kanit text-xl font-bold italic text-zinc-300">
                            RANKING
                        </div>
                        <div class="font-kanit text-sm text-zinc-300 border-b border-b-secondary-gray">
                            {format!("Player count: {:?}", players.len())}
                        </div>
                        <For
                            each=move || players.clone().into_iter().enumerate()
                            key=|(_, player_data)| player_data.username.clone()
                            children=move |(_, player_item)| {
                                let player_memo = create_memo(move |_| {
                                    player_item.clone()
                                });
                                view! {
                                    <Player player_data=player_memo() />
                                }
                            }
                        />
                    </div>
                }
            })}
        </Suspense>
    }
}

#[component]
pub fn Player(player_data: Player) -> impl IntoView {
    log!("Player: {}", player_data);
    let mut profile_pic = String::from("https://raw.githubusercontent.com/Ikken9/pencaucu/dev/cdn/default.jpg");
    if let Some(pic) = player_data.profile_picture {
        if !pic.is_empty() {
            profile_pic = pic;
        }
    }

    view! {
        <div class="flex items-center justify-between w-full match-card bg-gradient-to-r from-primary-gray-1 to-primary-gray-2 p-2 rounded-lg shadow-md mb-1 sm:p-4 h-14">
            <div class="flex items-center relative z-10">
                <img class="h-10 w-10 rounded-full mr-2" src={profile_pic} />
                <div class="text-xl text-zinc-300 ">
                    {player_data.username}
                </div>
            </div>
                <div class="font-kanit text-xl font-bold italic text-zinc-300">
                    {format!("{} PTS", player_data.points)}
                </div>
        </div>
    }
}