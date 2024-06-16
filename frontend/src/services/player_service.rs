use reqwest::Client;
use crate::models::player_model::Player;

pub async fn get_players() -> Result<Vec<Player>, reqwest::Error> {
    let response: Vec<Player> = Client::new()
        .get("http://localhost:8080/players")
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}

pub async fn get_player(id: &str) -> Result<Player, reqwest::Error> {
    let response: Player = Client::new()
        .get(format!("http://localhost:8080/players/{}", id))
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}