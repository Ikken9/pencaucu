mod models;
mod services;
mod routes;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::routes::admin_route::AdminPanel;
use crate::routes::bets_route::Bet;
use crate::routes::matches_route::*;
use crate::routes::login_route::*;
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
            <nav>
            /* ... */
            </nav>
            <main>
                <Routes>
                    <Route path="/login" view=Login/>
                    <Route path="/register" view=Register/>
                    <Route path="/matches" view=Matches/>
                    <Route path="/matches/:id/bet" view=Bet/>
                    <Route path="/ranking" view=Ranking/>
                    <Route path="/admin-panel" view=AdminPanel/>
                    <Route path="/admin-panel/upload-match" view=UploadMatch/>
                    <Route path="/admin-panel/upload-result" view=UploadResult/>
                    <Route path="/bets/:email/:match-id" view=Bet/>
                    <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}
