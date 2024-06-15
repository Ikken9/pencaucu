use leptos::*;
use leptos_router::*;
use crate::models::match_model::Match;

#[component]
pub fn Bet(match_data: Match) -> impl IntoView {
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