use std::collections::HashMap;
use leptos_router::Params;
use reqwest::{Client, Response};
use crate::models::bet_model::Bet;

pub async fn make_bet(player_email: &str, match_id: &u32, team_score: &u8, faced_team_score: &u8) -> Result<Response, reqwest::Error> {
    let token = web_sys::window().unwrap().local_storage().unwrap().unwrap().get_item("token").unwrap();
    let mut map = HashMap::new();
    map.insert("playerEmail", player_email.to_string());
    map.insert("matchId", match_id.to_string());
    map.insert("teamScore", team_score.to_string());
    map.insert("facedTeamScore", faced_team_score.to_string());

    let client = Client::new();

    let req_builder = client.post(format!("http://localhost:8080/bets/{}/{}", player_email, match_id));

    let req = if let Some(token) = token {
        req_builder.bearer_auth(token).json(&map)
    } else {
        req_builder
    };

    let res = req.send().await;

    match res {
        Ok(response) => {
            Ok(response)
        }
        Err(e) => {
            Err(e)
        }
    }
}

pub async fn get_bets_by_player(player_email: &str) -> Result<Vec<Bet>, reqwest::Error> {
    let token = web_sys::window().unwrap().local_storage().unwrap().unwrap().get_item("token").unwrap();
    let client = Client::new();

    let req_builder = client.get(format!("http://localhost:8080/bets/{}", player_email));

    let req = if let Some(token) = token {
        req_builder.bearer_auth(token)
    } else {
        req_builder
    };

    let res = req.send().await?;

    if res.status().is_success() {
        let bets = res.json::<Vec<Bet>>().await?;
        Ok(bets)
    } else {
        Err(res.error_for_status().unwrap_err())
    }
}

#[derive(leptos::Params, PartialEq)]
pub struct BetParams {
    pub(crate) match_id: Option<String>
}
