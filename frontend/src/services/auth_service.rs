use std::collections::HashMap;
use log::{error, info};
use reqwest::{Client, Response};
use crate::models::auth_model::{EmailAddress, Password, Username};

pub async fn login(email: EmailAddress, password: Password) -> Result<Response, reqwest::Error> {
    let mut map = HashMap::new();
    map.insert("email", email.to_string());
    map.insert("password", password.to_string());

    let client = Client::new();
    let res = client.post("http://localhost:8080/auth/login")
        .json(&map)
        .send()
        .await;

    match res {
        Ok(response) => {
            match response.text().await {
                Ok(text) => {
                    info!("Login response: {}", text);
                    // Rebuild the response if necessary (e.g., if you need to inspect it elsewhere)
                    let rebuilt_response = client.post("http://localhost:8080/auth/login")
                        .json(&map)
                        .send()
                        .await?;
                    Ok(rebuilt_response)
                },
                Err(e) => {
                    error!("Failed to read response text: {}", e);
                    Err(e)
                }
            }
        }
        Err(e) => {
            error!("Login request failed: {}", e);
            Err(e)
        }
    }
}

pub async fn register(username: Username, email_address: EmailAddress, password: Password) -> Result<Response, reqwest::Error> {
    let mut map = HashMap::new();
    map.insert("name", username.to_string());
    map.insert("email", email_address.to_string());
    map.insert("password", password.to_string());

    let client = Client::new();

    let res = client.post("http://localhost:8080/auth/register")
        .json(&map)
        .send()
        .await;

    match res {
        Ok(response) => {
            match response.text().await {
                Ok(text) => {
                    info!("Register response: {}", text);
                    // Rebuild the response if necessary (e.g., if you need to inspect it elsewhere)
                    let rebuilt_response = client.post("http://localhost:8080/auth/register")
                        .json(&map)
                        .send()
                        .await?;
                    Ok(rebuilt_response)
                },
                Err(e) => {
                    error!("Failed to read response text: {}", e);
                    Err(e)
                }
            }
        }
        Err(e) => {
            error!("Register request failed: {}", e);
            Err(e)
        }
    }
}