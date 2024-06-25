use std::collections::HashMap;
use reqwest::{Client, Response};
use crate::models::auth_model::{EmailAddress, Password, Username};
use crate::models::career_model::Career;

pub async fn register(username: Username, email_address: EmailAddress, career: Career, champion: String, subchampion: String, password: Password) -> Result<Response, reqwest::Error> {
    let mut map = HashMap::new();
    map.insert("username", username.to_string());
    map.insert("email", email_address.to_string());
    map.insert("career", career.to_string());
    map.insert("champion", champion);
    map.insert("subchampion", subchampion);
    map.insert("password", password.to_string());

    let client = Client::new();

    let res = client.post("http://localhost:8080/register")
        .json(&map)
        .send()
        .await;

    match res {
        Ok(response) => {
            Ok(response)
        }
        Err(e) => {
            Err(e)
        }
    }
}