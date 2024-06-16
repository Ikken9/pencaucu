use reqwest::Client;
use crate::models::match_model::Match;

pub async fn get_matches() -> Result<Vec<Match>, reqwest::Error> {
    let response: Vec<Match> = Client::new()
        .get("http://localhost:8080/matches")
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}

pub async fn get_match(id: &str) -> Result<Match, reqwest::Error> {
    let response: Match = Client::new()
        .get(format!("http://localhost:8080/matches/{}", id))
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}