use leptos::*;
use leptos_router::*;
use crate::services::match_service;

#[component]
pub fn Bet() -> impl IntoView {
    let params = use_params_map();
    let match_data = create_resource(
        move || params.get().get("id").cloned().unwrap_or_default(),
        move |id| async move {
            if id.is_empty() {
                None
            } else {
                match_service::get_match(&id).await.ok()
            }
        },
    );

    view! {
        <Suspense fallback=|| view! { "Loading match..." }>
            {move || match_data.get().map(|match_data| match match_data {
                None => view! { <div>"Error loading match data."</div> },
                Some(match_data) => view! {
                    <div class="match-view">
                        <h1>{format!("Match ID: {}", match_data.id)}</h1>
                        <p>{format!("Date: {}", match_data.date)}</p>
                        <p>{format!("Team 1 Name: {}", match_data.first_team_name)}</p>
                        <p>{format!("Team 2 Name: {}", match_data.second_team_name)}</p>
                        <p>{format!("Team 1 Score: {}", match_data.first_team_score)}</p>
                        <p>{format!("Team 2 Score: {}", match_data.second_team_score)}</p>
                    </div>
                }
            })}
        </Suspense>
    }
}