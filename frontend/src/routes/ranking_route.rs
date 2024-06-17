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
                    <div class="players-container">
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
    view! {
        <div class="flex items-center space-x-4 rtl:space-x-reverse">
         <div class="flex-1 min-w-0">
            <p class="text-sm font-medium text-gray-900 truncate dark:text-white">
               {player_data.username}
            </p>
         </div>
         <div class="inline-flex items-center text-base font-semibold text-gray-900 dark:text-white">
            {player_data.points}
         </div>
      </div>
    }
}