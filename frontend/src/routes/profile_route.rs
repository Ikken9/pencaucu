use leptos::*;
use leptos::leptos_dom::{error, log};
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
                    view! {
                        <div class="flex flex-col items-center justify-center h-screen bg-gray-100">
                            <div class="bg-white shadow-md rounded-lg p-8 max-w-sm w-full">
                                <div class="flex flex-col items-center">
                                    <img class="h-24 w-24 rounded-full object-cover" src={player.profile_picture} alt="profile-picture" />
                                    <h2 class="mt-4 text-2xl font-semibold text-gray-900">"John Doe"</h2>
                                    <p class="mt-2 text-gray-600">"john.doe@example.com"</p>
                                </div>
                                <div class="mt-8">
                                    <button class="w-full bg-red-500 hover:bg-red-600 text-white py-2 px-4 rounded-md">
                                        "Logout"
                                    </button>
                                </div>
                            </div>
                        </div>
                    }
                }})};
        </Suspense>
    }
}