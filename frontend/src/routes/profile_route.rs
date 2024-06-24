use leptos::*;
use leptos::leptos_dom::{error, log};
use crate::Navbar;
use crate::services::player_service;

#[component]
pub fn Profile() -> impl IntoView {
    let player_data = create_resource(
        || (),  // The initial state for the resource
        move |_| {
            let email = web_sys::window().unwrap().local_storage().unwrap().unwrap().get_item("email").unwrap().unwrap();
            async move {
                log!("Fetching player...");
                let result = player_service::get_player(&*email.clone()).await;
                match result {
                    Ok(player) => {
                        log!("Successfully fetched player data.");
                        Some(player)
                    }
                    Err(e) => {
                        error!("Error fetching player data: {:?}", e);
                        None
                    }
                }
            }
        },
    );

    view! {
        <Suspense fallback=|| view! { "Loading player..." }>
            { move || player_data.get().map(|p| match p {
                None => view! { <div>"Error loading player."</div> },
                Some(player) => {
                    let email = web_sys::window().unwrap().local_storage().unwrap().unwrap().get_item("email").unwrap().unwrap();
                    view! {
                        <div class="flex flex-col items-center justify-center">
                            <div class="bg-gradient-to-r from-primary-gray-1 to-primary-gray-2 shadow-md rounded-lg p-8 max-w-sm w-full mt-8">
                                <div class="flex flex-col items-center">
                                    <img class="border-4 border-purple-700 h-24 w-24 rounded-full object-cover" src={player.profile_picture} alt="profile-picture" />
                                    <h2 class="mt-4 text-2xl font-semibold text-zinc-300">{player.username}</h2>
                                    <p class="mt-2 text-zinc-300">{&email.clone()}</p>
                                </div>
                            </div>
                        </div>
                    }
                }})};
        </Suspense>
        <Navbar/>
    }
}