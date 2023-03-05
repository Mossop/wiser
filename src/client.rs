use reqwest::Client as HttpClient;
use serde::de::DeserializeOwned;

use crate::Error;

#[derive(Debug, Clone)]
pub struct Client {
    secret: String,
    host: String,
    client: HttpClient,
}

impl Client {
    pub fn new(host: &str, secret: &str) -> Self {
        Self {
            secret: secret.to_owned(),
            host: host.to_owned(),
            client: HttpClient::new(),
        }
    }

    pub async fn get<T>(&self, path: &str) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let url = format!("http://{}/data/v2{}", self.host, path);
        log::trace!("Requesting {}...", url);

        let data = self
            .client
            .get(url)
            .header("SECRET", &self.secret)
            .header("Content-Type", "application/json")
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        match serde_json::from_str(&data) {
            Ok(result) => Ok(result),
            Err(e) => {
                log::error!("Failed to decode API response: {}", e);
                eprintln!("{data}");
                Err(e.into())
            }
        }
    }
}
