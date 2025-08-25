mod response;

use {
    anyhow::{Context as _, Result},
    log::info,
};

pub(crate) struct ChargePoint {
    client: reqwest::Client,
    url: reqwest::Url,
    token: String,
}

impl ChargePoint {
    pub(crate) async fn login(configuration: &crate::configuration::ChargePoint) -> Result<Self> {
        info!("Logging in to {}", configuration.url);

        let client = reqwest::Client::new();

        let url = reqwest::Url::parse(&configuration.url)
            .with_context(|| format!("Could not parse {}", configuration.url))?;

        let response: response::login::Root = client
            .post(url.join("api/v1/AuthManagement/Login").unwrap())
            .json(&serde_json::json!({ "username": configuration.username, "password": configuration.password }))
            .send()
            .await
            .context("Could not send login request")?
            .json()
            .await
            .context("Could not decode login response")?;

        info!(
            "Successfully logged in to {}. The token is:\n{}",
            configuration.url, response.token
        );

        Ok(Self {
            client,
            url,
            token: response.token,
        })
    }
}
