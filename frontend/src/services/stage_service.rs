use reqwest::Client;
use crate::models::stage_model::KnockoutStage;

pub async fn get_knockout_stages() -> Result<Vec<KnockoutStage>, reqwest::Error> {
    let token = web_sys::window().unwrap().session_storage().unwrap().unwrap().get_item("token").unwrap();
    let client = Client::new();

    let req_builder = client.get("http://localhost:8080/stages");

    let req = if let Some(token) = token {
        req_builder.bearer_auth(token)
    } else {
        req_builder
    };

    let res = req.send().await?;

    if res.status().is_success() {
        let stages = res.json::<Vec<KnockoutStage>>().await?;
        Ok(stages)
    } else {
        Err(res.error_for_status().unwrap_err())
    }
}