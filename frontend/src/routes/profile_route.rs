use leptos::*;
use leptos::leptos_dom::{error, log};
use crate::services::player_service;

// #[component]
// pub fn Profile() -> impl IntoView {
//     let email = web_sys::window().unwrap().local_storage().unwrap().unwrap().get_item("email").unwrap();
//
//     let player_data = create_resource(
//         || (),  // The initial state for the resource
//         |_| async {
//             log!("Fetching player...");
//             let result = player_service::get_player("").await;
//             match result {
//                 Ok(players) => {
//                     log!("Successfully fetched player data.");
//                     Some(players)
//                 }
//                 Err(e) => {
//                     error!("Error fetching player data: {:?}", e);
//                     None
//                 }
//             }
//         },
//     );
//
//     view! {
//         <div class="flex flex-col items-center justify-center h-screen bg-gray-100">
//             <div class="bg-white shadow-md rounded-lg p-8 max-w-sm w-full">
//                 <div class="flex flex-col items-center">
//                     <img class="h-24 w-24 rounded-full object-cover" src="https://via.placeholder.com/150" alt="User Profile Picture" />
//                     <h2 class="mt-4 text-2xl font-semibold text-gray-900">"John Doe"</h2>
//                     <p class="mt-2 text-gray-600">"john.doe@example.com"</p>
//                 </div>
//                 <div class="mt-8">
//                     <button class="w-full bg-red-500 hover:bg-red-600 text-white py-2 px-4 rounded-md">
//                         "Logout"
//                     </button>
//                 </div>
//             </div>
//         </div>
//     }
// }