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
            <main>
                <Routes>
                    <Route path="/" view=Register/>
                    <Route path="/login" view=Login/>
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
        <nav class="fixed bottom-0 left-0 right-0 bg-gradient-to-r from-indigo-600 to-violet-600 rounded-2xl">
            <div class="max-w-7xl mx-auto px-2 sm:px-6 lg:px-8">
                <div class="flex justify-between h-16">
                    <a href="/matches" class="flex flex-col items-center justify-center text-zinc-300 hover:bg-gray-700 py-2 rounded-md text-sm font-medium">
                        <img src="https://raw.githubusercontent.com/Ikken9/pencaucu/dev/cdn/ball.svg" alt="Betting Icon" class="w-6 h-6"/>
                        <span>"Matches"</span>
                    </a>
                    <a href="/bets" class="flex flex-col items-center justify-center text-zinc-300 hover:bg-gray-700 px-3 py-2 rounded-md text-sm font-medium">
                        <img src="https://raw.githubusercontent.com/Ikken9/pencaucu/dev/cdn/bet.svg" alt="Betting Icon" class="w-6 h-6"/>
                        <span>"Bets"</span>
                    </a>
                    <a href="/ranking" class="flex flex-col items-center justify-center text-zinc-300 hover:bg-gray-700 px-3 py-2 rounded-md text-sm font-medium">
                        <img src="https://raw.githubusercontent.com/Ikken9/pencaucu/dev/cdn/ranking.svg" alt="Betting Icon" class="w-6 h-6"/>
                        <span>"Ranking"</span>
                    </a>
                    <a href="/profile" class="flex flex-col items-center justify-center text-zinc-300 hover:bg-gray-700 px-3 py-2 rounded-md text-sm font-medium">
                        <img src="https://raw.githubusercontent.com/Ikken9/pencaucu/dev/cdn/profile2.svg" alt="Betting Icon" class="w-6 h-6"/>
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
