pub async fn fetch_api<T>(url: &str) -> Result<T, reqwest::Error>
    where
        T: serde::de::DeserializeOwned,
{
    let response = reqwest::get(url).await?;
    let data = response.json::<T>().await?;
    Ok(data)
}

pub fn match_url(id: &str) -> String {
    format!("http://localhost:8080/matches/{}", id)
}

pub fn matches_url() -> String {
    "http://localhost:8080/matches".to_string()
}