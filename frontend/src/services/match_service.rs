use std::collections::HashMap;
use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
use leptos::logging::log;
use reqwest::{Client, Response};
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

pub async fn get_match(id: String) -> Result<Match, reqwest::Error> {
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

pub async fn upload_match(team_name: String, faced_team_name: String, date: String, stage_name: String, stadium_id: String) -> Result<Response, reqwest::Error> {
    let token = web_sys::window().unwrap().local_storage().unwrap().unwrap().get_item("token").unwrap();
    let timestamp = input_date_to_timestamp(date);
    let mut map = HashMap::new();
    map.insert("date", timestamp);
    map.insert("knockoutStageId", stage_name);
    map.insert("stadiumId", stadium_id);
    map.insert("teamName", team_name);
    map.insert("facedTeamName", faced_team_name);
    map.insert("adminEmail", String::from("martin.caraballo@correo.ucu.edu.uy"));

    let client = Client::new();

    let req_builder = client.post("http://localhost:8080/matches");

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

pub fn timestamp_to_date(timestamp: i64) -> DateTime<FixedOffset> {
    // Convert the timestamp to seconds
    let timestamp_secs = timestamp / 1000;

    // Convert the timestamp to a NaiveDateTime
    let naive_datetime = NaiveDateTime::from_timestamp(timestamp_secs, 0);

    // Specify the timezone (UTC in this case)
    let datetime_utc: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive_datetime, Utc);

    // Convert the datetime to the local timezone (for Uruguay, it is UTC-3)
    let datetime = datetime_utc.with_timezone(&chrono::FixedOffset::west(3 * 3600));

    datetime
}

pub fn input_date_to_timestamp(datetime: String) -> String {
    let pre_processed = format!("{}:00+03:00", datetime);

    log!("{}", pre_processed.clone());
    let datetime = DateTime::parse_from_rfc3339(&*pre_processed).unwrap();
    let datetime_utc = datetime.with_timezone(&Utc);

    let timestamp_secs = datetime_utc.timestamp();

    let timestamp_millis = timestamp_secs * 1000;
    timestamp_millis.to_string()
}