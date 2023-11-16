use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use anyhow::{anyhow, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub chat: Chat,
    pub from: User,
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Update {
    pub message: Message,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Chat {
    pub id: i64,
}

pub struct TeleApi {
    token: String,
    client: Client,
}

impl TeleApi {
    pub fn new(token: &str) -> TeleApi {
        TeleApi {
            token: token.to_string(),
            client: Client::new(),
        }
    }

    pub async fn get_updates(&self) -> Result<Vec<Update>, Error> {
        let method = "getUpdates?offset=-1";
        let response = self
            .client
            .post(&format!("https://api.telegram.org/bot{}/{}", self.token, method))
            .send()
            .await?;
        let updates: Vec<Update> = response.json().await?;
        Ok(updates)
    }

    pub async fn execute(&self, method: &str, params: &[(&str, &str)]) -> Result<(), anyhow::Error> {
        let url = format!("https://api.telegram.org/bot{}/{}", self.token, method);

        let mut form_params = params.to_owned();
        form_params.push(("method", method));

        let response = self
            .client
            .post(&url)
            .form(&form_params)
            .send()
            .await?;

        let response_text = response.text().await?;
        let json: serde_json::Value = serde_json::from_str(&response_text)?;

        if json["ok"].as_bool().unwrap_or(false) {
            Ok(())
        } else {
            Err(anyhow!(
                "Telegram API Error: {}",
                json["description"].as_str().unwrap_or("Unknown error")
            ))
        }
    }
}