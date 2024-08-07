use anyhow::{anyhow, Error, Ok};
use reqwest::{Client, Response};
use serde_derive::Deserialize;

const BASE_URL: &str = "https://api.usercheck.com";

#[derive(Debug, Deserialize)]
pub struct DomainInfo {
    pub status: u8,
    pub domain: String,
    pub mx: bool,
    pub disposable: bool,
    pub public_domain: bool,
    pub did_you_mean: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct EmailInfo {
    pub status: u8,
    pub email: String,
    pub domain: String,
    pub mx: bool,
    pub disposable: bool,
    pub public_domain: bool,
    pub alias: bool,
    pub did_you_mean: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct ErrorInfo {
    pub status: u16,
    pub error: String
}

pub struct UserCheckClient {
    client: Client,
    api_key: Option<String>
}

impl UserCheckClient {
    pub fn new(api_key: Option<String>) -> Self {
        let client = Client::new();
        UserCheckClient { client, api_key }
    }

    async fn make_request(&self, endpoint: &str) -> Result<Response, Error> {
        let url = format!("{}/{}", BASE_URL, endpoint);

        let response = if let Some(ref key) = self.api_key {
            self.client.get(url)
                .header("Authorization".to_string(), format!("Bearer {}", key))
                .send().await?
        } else {
            self.client.get(url).send().await?
        };

        if !response.status().is_success() {
            let error_info: ErrorInfo = response.json().await?;
            Err(anyhow!("{}", error_info.error))
        } else {
            Ok(response)
        }
    }

    pub async fn check_domain(&self, domain: &str) -> Result<DomainInfo, Error> {
        let endpoint = format!("domain/{}", domain);
        let response = self.make_request(&endpoint).await?;
        let domain_info = response.json().await?;
        Ok(domain_info)
    }

    pub async fn check_email(&self, email: &str) -> Result<EmailInfo, Error> {
        let endpoint = format!("email/{}", email);
        let response = self.make_request(&endpoint).await?;
        let email_info = response.json().await?;
        Ok(email_info)
    }
}
