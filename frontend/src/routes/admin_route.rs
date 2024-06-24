use leptos::*;
use leptos_router::*;
use crate::Navbar;

#[component]
pub fn AdminPanel() -> impl IntoView {
    view! {
        <div class="p-3">
            <div class= "font-kanit text-xl font-bold italic text-zinc-300">
            ADMIN PANEL
            </div>
            <div class="container">
                <div class="load-match-card bg-gradient-to-r from-primary-gray-1 to-primary-gray-2 p-2 rounded-lg shadow-md flex flex-col items-center mb-1 sm:p-4 h-15 text-zinc-300">
                    <A href="/admin-panel/upload-match">"Load match"</A>
                </div>
                <div class="load-match-result-card bg-gradient-to-r from-primary-gray-1 to-primary-gray-2 p-2 rounded-lg shadow-md flex flex-col items-center mb-1 sm:p-4 h-15 text-zinc-300">
                    <A href="/admin-panel/upload-result">"Load match result"</A>
                </div>
            </div>
        </div>
        <Navbar/>
    }
}