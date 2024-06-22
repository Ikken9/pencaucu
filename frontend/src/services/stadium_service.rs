use reqwest::Client;
use crate::models::stadium_model::Stadium;

pub async fn get_stadiums() -> Result<Vec<Stadium>, reqwest::Error> {
    let token = web_sys::window().unwrap().local_storage().unwrap().unwrap().get_item("token").unwrap();
    let client = Client::new();

    let req_builder = client.get("http://localhost:8080/stadiums");

    let req = if let Some(token) = token {
        req_builder.bearer_auth(token)
    } else {
        req_builder
    };

    let res = req.send().await?;

    if res.status().is_success() {
        let stadiums = res.json::<Vec<Stadium>>().await?;
        Ok(stadiums)
    } else {
        Err(res.error_for_status().unwrap_err())
    }
}