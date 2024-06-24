use std::collections::HashMap;
use reqwest::{Client, Response};
use crate::models::result_model::MatchResult;
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

pub async fn get_result_by_id(id: &u32) -> Result<Vec<MatchResult>, reqwest::Error> {
    let token = web_sys::window().unwrap().local_storage().unwrap().unwrap().get_item("token").unwrap();
    let client = Client::new();

    let req_builder = client.get(format!("http://localhost:8080/results/{}", id));

    let req = if let Some(token) = token {
        req_builder.bearer_auth(token)
    } else {
        req_builder
    };

    let res = req.send().await?;

    if res.status().is_success() {
        let pending_results = res.json::<Vec<MatchResult>>().await?;
        Ok(pending_results)
    } else {
        Err(res.error_for_status().unwrap_err())
    }
}

pub async fn submit_result(match_id: &u32, team_score: &u8, faced_team_score: &u8) -> Result<Response, reqwest::Error> {
    let token = web_sys::window().unwrap().local_storage().unwrap().unwrap().get_item("token").unwrap();
    let mut map = HashMap::new();
    map.insert("matchId", match_id.to_string());
    map.insert("teamScore", team_score.to_string());
    map.insert("facedTeamScore", faced_team_score.to_string());

    let client = Client::new();

    let req_builder = client.post("http://localhost:8080/results");

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

pub async fn edit_result(match_id: &u32, team_score: &u8, faced_team_score: &u8) -> Result<Response, reqwest::Error> {
    let token = web_sys::window().unwrap().local_storage().unwrap().unwrap().get_item("token").unwrap();
    let mut map = HashMap::new();
    map.insert("matchId", match_id.to_string());
    map.insert("teamScore", team_score.to_string());
    map.insert("facedTeamScore", faced_team_score.to_string());

    let client = Client::new();

    let req_builder = client.put("http://localhost:8080/results");

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