use leptos::logging::log;
use reqwest::Client;
use crate::models::bet_model::Bet;


pub async fn make_bet(player_email: &str, match_id: &u64) -> Result<Bet, reqwest::Error> {
    let token = web_sys::window().unwrap().local_storage().unwrap().unwrap().get_item("token").unwrap();
    let client = Client::new();

    let url = format!("http://localhost:8080/bets/{}/{}", player_email, match_id);
    let req_builder = client.get(&url);

    let req = if let Some(token) = token {
        req_builder.bearer_auth(token)
    } else {
        req_builder
    };

    let res = req.send().await?;

    if res.status().is_success() {
        let bet = res.json::<Bet>().await?;
        Ok(bet)
    } else {
        Err(res.error_for_status().unwrap_err())
    }
}