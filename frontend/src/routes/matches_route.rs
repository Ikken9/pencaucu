use leptos::*;
use leptos::logging::{error, log};
use leptos_router::*;
use crate::models::match_model::Match;
use crate::services::match_service;

#[component]
pub fn Match(match_data: Match) -> impl IntoView {
    view! {
        <div class="match-card">
            <A href=format!("/match/bet/{:?}", match_data.id)>
                <p>xd</p>
            </A>
            <p>{format!("Date: {}", match_data.date)}</p>
            <p>{format!("Team 1 Name: {}", match_data.first_team_name)}</p>
            <p>{format!("Team 2 Name: {}", match_data.second_team_name)}</p>
            <p>{format!("Team 1 Score: {}", match_data.first_team_score)}</p>
            <p>{format!("Team 2 Score: {}", match_data.second_team_score)}</p>
        </div>
    }
}

#[component]
pub fn Matches() -> impl IntoView {
    let matches_data = create_resource(
        || (),  // The initial state for the resource
        |_| async {
            log!("Fetching matches...");
            let result = match_service::get_matches().await;
            match result {
                Ok(matches) => {
                    log!("Successfully fetched matches.");
                    Some(matches)
                }
                Err(e) => {
                    error!("Error fetching matches: {:?}", e);
                    None
                }
            }
        },
    );

    view! {
        <Suspense fallback=|| view! { "Loading matches..." }>
            {move || matches_data.get().map(|matches| match matches {
                None => view! { <div>"Error loading matches."</div> },
                Some(matches) => view! {
                    <div class="matches-container">
                        <For
                            each=move || matches.clone().into_iter().enumerate()
                            key=|(_, match_data)| match_data.id.clone()
                            children=move |(_, match_item)| {
                                let match_memo = create_memo(move |_| {
                                    match_item.clone()
                                });
                                view! {
                                    <Match match_data=match_memo() />
                                }
                            }
                        />
                    </div>
                }
            })}
        </Suspense>
    }
}