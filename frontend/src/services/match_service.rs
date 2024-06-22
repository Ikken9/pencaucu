use std::time::{Duration, UNIX_EPOCH};
use chrono::{DateTime, Local};
use reqwest::Client;
use crate::models::match_model::Match;

pub async fn get_matches() -> Result<Vec<Match>, reqwest::Error> {
    let token = web_sys::window().unwrap().local_storage().unwrap().unwrap().get_item("token").unwrap();
    let client = Client::new();

    let req_builder = client.get("http://localhost:8080/matches");

    let req = if let Some(token) = token {
        req_builder.bearer_auth(token)
    } else {
        req_builder
    };

    let res = req.send().await?;

    if res.status().is_success() {
        let matches = res.json::<Vec<Match>>().await?;
        Ok(matches)
    } else {
        Err(res.error_for_status().unwrap_err())
    }
}

pub async fn get_match(id: &str) -> Result<Match, reqwest::Error> {
    let token = web_sys::window().unwrap().local_storage().unwrap().unwrap().get_item("token").unwrap();
    let client = Client::new();

    let req_builder = client.get(format!("http://localhost:8080/matches/{}", id));

    let req = if let Some(token) = token {
        req_builder.bearer_auth(token)
    } else {
        req_builder
    };

    let res = req.send().await?;

    if res.status().is_success() {
        let m = res.json::<Match>().await?;
        Ok(m)
    } else {
        Err(res.error_for_status().unwrap_err())
    }
}

pub fn u64_to_date(timestamp: u64) -> DateTime<Local> {
    // Convert u64 timestamp (milliseconds since UNIX_EPOCH) to SystemTime
    let unix_epoch = UNIX_EPOCH;
    let system_time = unix_epoch + Duration::from_millis(timestamp);

    // Convert SystemTime to DateTime<Local> (or other timezone if needed)
    DateTime::<Local>::from(system_time)
}