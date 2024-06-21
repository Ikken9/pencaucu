use reqwest::Client;

pub async fn make_bet(match_id: &str) -> Result<(), reqwest::Error> {
    let token = web_sys::window().unwrap().local_storage().unwrap().unwrap().get_item("token").unwrap();
    let client = Client::new();

    let url = format!("http://localhost:8080/matches/{}", match_id);
    let request = client.post(&url);
    let request = if let Some(token) = token {
        request.bearer_auth(token)
    } else {
        request
    };

    let response = request.send().await?;
    if response.status().is_success() {
        log!("Bet request successful.");
        Ok(())
    } else {
        log!("Bet request failed with status: {}", response.status());
        Err(reqwest::Error::new(
            reqwest::StatusCode::INTERNAL_SERVER_ERROR,
            "Bet request failed",
        ))
    }
}