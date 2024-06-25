use reqwest::Client;

pub async fn get_teams_names() -> Result<Vec<String>, reqwest::Error> {
    let token = web_sys::window().unwrap().session_storage().unwrap().unwrap().get_item("token").unwrap_or(None);
    let client = Client::new();

    let req_builder = client.get("http://localhost:8080/teams/names");

    let req = if let Some(token) = token {
        req_builder.bearer_auth(token)
    } else {
        req_builder
    };

    let res = req.send().await?;

    if res.status().is_success() {
        let teams = res.json::<Vec<String>>().await?;
        Ok(teams)
    } else {
        Err(res.error_for_status().unwrap_err())
    }
}