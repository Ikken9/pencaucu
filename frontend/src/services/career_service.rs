use reqwest::Client;
use crate::models::auth_model::Career;

pub async fn get_careers() -> Result<Vec<Career>, reqwest::Error> {
    let response: Vec<Career> = Client::new()
        .get("http://localhost:8080/careers")
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}