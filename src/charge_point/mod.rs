mod response;
mod status;

use {
    anyhow::{Context as _, Ok, Result},
    log::info,
    status::Status,
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

    pub(crate) async fn status(&self) -> Result<Status> {
        let response: response::status::Root = self
            .client
            .get(self.url.join("api/v1/Configuration/GetConfigurationPage?guid=6C0BE508-4ADE-4CB5-8C08-76CB4527CD89").unwrap())
            .bearer_auth(&self.token)
            .send()
            .await
            .context("Could not status response")?
            .json()
            .await
            .context("Could not decode status response")?;

        Ok(response.into())
    }
}
