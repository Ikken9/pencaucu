mod models;
mod services;
mod routes;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::routes::admin_route::AdminPanel;
use crate::routes::bets_route::{Bets, MakeBet};
use crate::routes::matches_route::*;
use crate::routes::login_route::*;
use crate::routes::profile_route::Profile;
use crate::routes::ranking_route::Ranking;
use crate::routes::register_route::*;
use crate::routes::upload_match_route::UploadMatch;
use crate::routes::upload_result_route::UploadResult;


#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let (is_routing, set_is_routing) = create_signal(false);
    view! {
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Meta name="description" content="PencaUCU App"/>
        // adding `set_is_routing` causes the router to wait for async data to load on new pages
        <Router set_is_routing>
            <div class="routing-progress">
                <RoutingProgress is_routing max_time=std::time::Duration::from_millis(250)/>
            </div>
            <Navbar/>
            <main>
                <Routes>
                    <Route path="/login" view=Login/>
                    <Route path="/register" view=Register/>
                    <Route path="/matches" view=Matches/>
                    <Route path="/bets/bet/:match-id" view=MakeBet/>
                    <Route path="/bets" view=Bets/>
                    <Route path="/ranking" view=Ranking/>
                    <Route path="/profile" view=Profile/>
                    <Route path="/admin-panel" view=AdminPanel/>
                    <Route path="/admin-panel/upload-match" view=UploadMatch/>
                    <Route path="/admin-panel/upload-result" view=UploadResult/>
                    <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="fixed bottom-0 left-0 right-0 bg-gray-800">
            <div class="max-w-7xl mx-auto px-2 sm:px-6 lg:px-8">
                <div class="flex justify-between h-16">
                    <a href="/matches" class="flex flex-col items-center justify-center text-gray-300 hover:bg-gray-700 hover:text-white px-3 py-2 rounded-md text-sm font-medium">
                        <svg class="h-6 w-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16m-7 6h7" />
                        </svg>
                        <span>"Matches"</span>
                    </a>
                    <a href="/bets" class="flex flex-col items-center justify-center text-gray-300 hover:bg-gray-700 hover:text-white px-3 py-2 rounded-md text-sm font-medium">
                        <svg class="h-6 w-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16m-7 6h7" />
                        </svg>
                        <span>"Bets"</span>
                    </a>
                    <a href="/ranking" class="flex flex-col items-center justify-center text-gray-300 hover:bg-gray-700 hover:text-white px-3 py-2 rounded-md text-sm font-medium">
                        <svg class="h-6 w-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 11h18M3 16h18M3 6h18" />
                        </svg>
                        <span>"Ranking"</span>
                    </a>
                    <a href="/profile" class="flex flex-col items-center justify-center text-gray-300 hover:bg-gray-700 hover:text-white px-3 py-2 rounded-md text-sm font-medium">
                        <svg class="h-6 w-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5.121 18.879A3.5 3.5 0 018 18h8a3.5 3.5 0 013.5 3.5V21H5v-2.121a3.5 3.5 0 01.121-1.879zM15 13a3 3 0 110-6 3 3 0 010 6z" />
                        </svg>
                        <span>"Profile"</span>
                    </a>
                </div>
            </div>
        </nav>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}
