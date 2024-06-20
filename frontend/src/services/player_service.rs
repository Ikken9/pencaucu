use reqwest::Client;
use crate::models::player_model::Player;

pub async fn get_players() -> Result<Vec<Player>, reqwest::Error> {
    let token = web_sys::window().unwrap().session_storage().unwrap().unwrap().get_item("token").unwrap_or(None);
    let client = Client::new();

    let req_builder = client.get("http://localhost:8080/players");

    let req = if let Some(token) = token {
        req_builder.bearer_auth(token)
    } else {
        req_builder
    };

    let res = req.send().await?;

    if res.status().is_success() {
        let players = res.json::<Vec<Player>>().await?;
        Ok(players)
    } else {
        Err(res.error_for_status().unwrap_err())
    }
}

pub async fn get_player(id: &str) -> Result<Player, reqwest::Error> {
    let token = web_sys::window().unwrap().session_storage().unwrap().unwrap().get_item("token").unwrap_or(None);
    let client = Client::new();

    let req_builder = client.get(format!("http://localhost:8080/players/{}", id));

    let req = if let Some(token) = token {
        req_builder.bearer_auth(token)
    } else {
        req_builder
    };

    let res = req.send().await?;

    if res.status().is_success() {
        let player = res.json::<Player>().await?;
        Ok(player)
    } else {
        Err(res.error_for_status().unwrap_err())
    }
}