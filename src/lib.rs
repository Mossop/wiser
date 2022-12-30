mod client;
mod error;

use client::Client;
pub use error::Error;
use serde::Deserialize;

pub struct Hub {
    client: Client,
}

#[derive(Deserialize)]
pub struct Domain {}

impl Hub {
    pub fn new(host: &str, secret: &str) -> Self {
        Self {
            client: Client::new(host, secret),
        }
    }

    pub async fn domain(&self) -> Result<Domain, Error> {
        self.client.get("/domain/").await
    }
}
