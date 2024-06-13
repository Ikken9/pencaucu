mod models;
mod services;
mod routes;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::routes::matches_route::*;
use crate::routes::ranking_route::Ranking;


#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let (is_routing, set_is_routing) = create_signal(false);

    view! {
        // <Stylesheet id="leptos" href="/pkg/hackernews.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Meta name="description" content="Leptos implementation of a HackerNews demo."/>
        // adding `set_is_routing` causes the router to wait for async data to load on new pages
        <Router set_is_routing>
            // shows a progress bar while async data are loading
            <div class="routing-progress">
                <RoutingProgress is_routing max_time=std::time::Duration::from_millis(250)/>
            </div>
            //<Nav />
            <main>
                <Routes>
                    <Route path=":match/:id" view=Match/>
                    <Route path=":matches?" view=Matches/>
                    <Route path=":ranking?" view=Ranking/>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}
