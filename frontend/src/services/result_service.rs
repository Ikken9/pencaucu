use reqwest::Client;
use crate::models::match_model::Match;

pub async fn get_pending_results() -> Result<Vec<Match>, reqwest::Error> {
    let token = web_sys::window().unwrap().local_storage().unwrap().unwrap().get_item("token").unwrap();
    let client = Client::new();

    let req_builder = client.get("http://localhost:8080/results/pending");

    let req = if let Some(token) = token {
        req_builder.bearer_auth(token)
    } else {
        req_builder
    };

    let res = req.send().await?;

    if res.status().is_success() {
        let pending_results = res.json::<Vec<Match>>().await?;
        Ok(pending_results)
    } else {
        Err(res.error_for_status().unwrap_err())
    }
}

pub async fn get_submitted_results() -> Result<Vec<Match>, reqwest::Error> {
    let token = web_sys::window().unwrap().local_storage().unwrap().unwrap().get_item("token").unwrap();
    let client = Client::new();

    let req_builder = client.get("http://localhost:8080/results/submitted");

    let req = if let Some(token) = token {
        req_builder.bearer_auth(token)
    } else {
        req_builder
    };

    let res = req.send().await?;

    if res.status().is_success() {
        let submitted_results = res.json::<Vec<Match>>().await?;
        Ok(submitted_results)
    } else {
        Err(res.error_for_status().unwrap_err())
    }
}