use leptos::*;
use leptos_router::*;


#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <nav>
            /* ... */
            </nav>
            <main>
            // all our routes will appear inside <main>
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/matches" view=Matches/>
                    <Route path="/matches/:id" view=Match/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Tabs(progress: ReadSignal<String>) -> impl IntoView {
    view! {

    }
}

#[component]
fn Tab(progress: ReadSignal<String>) -> impl IntoView {
    view! {

    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}
