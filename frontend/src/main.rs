mod models;
mod services;
mod routes;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::routes::matches_route::*;
use crate::routes::login_route::*;
use crate::routes::register_route::*;
use crate::services::bets_service::Bet;


#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let (is_routing, set_is_routing) = create_signal(false);
    println!("xd");
    view! {
        // <Stylesheet id="leptos" href="/pkg/hackernews.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Meta name="description" content="Leptos implementation of a HackerNews demo."/>
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
                    <Route path="/" view=Home/>
                    <Route path="/auth/login" view=Login/>
                    <Route path="/auth/register" view=Register/>
                    <Route path="/matches" view=Matches/>
                    <Route path="/matches/:id" view=Match/>
                    <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <h1>
            Home
        </h1>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}
