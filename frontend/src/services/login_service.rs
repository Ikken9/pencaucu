use std::collections::HashMap;
use reqwest::{Client, Response};
use crate::models::auth_model::{EmailAddress, Password};

pub async fn login(email: EmailAddress, password: Password) -> Result<Response, reqwest::Error> {
    let mut map = HashMap::new();
    map.insert("email", email.to_string());
    map.insert("password", password.to_string());

    let client = Client::new();
    let res = client.post("http://localhost:8080/login")
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