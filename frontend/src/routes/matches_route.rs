use leptos::*;
use leptos_router::*;
use crate::services::bets_service::Bet;
use crate::services::match_service;

#[component]
pub fn Match() -> impl IntoView {
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

#[component]
pub fn Matches() -> impl IntoView {
    let matches_data = create_resource(
        || (),
        |_| async {
            match_service::get_matches().await.ok()
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
                                    <Bet match_data=match_memo() />
                                }
                            }
                        />
                    </div>
                }
            })}
        </Suspense>
    }
}