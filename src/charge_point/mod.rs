mod response;

use {
    crate::{
        configuration::Property,
        database::{Record, Table},
    },
    anyhow::{Context as _, Ok, Result},
    log::info,
};

pub(crate) struct ChargePoint {
    client: reqwest::Client,
    url: reqwest::Url,
    token: String,
    observe: Vec<Property>,
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
            "Successfully logged in to {}. The token ends in: {}",
            configuration.url,
            &response.token[response.token.len() - 8..]
        );

        Ok(Self {
            client,
            url,
            token: response.token,
            observe: configuration.observe.clone(),
        })
    }

    pub(crate) async fn status(&self) -> Result<impl Iterator<Item = (Table, Record)>> {
        info!("Fetching status from {}", self.url);

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

        info!("Successfully fetched status from {}", self.url);

        Ok(self.observe.iter().filter_map(move |property| {
            response
                .configurations_groups
                .get(&property.group_id)
                .and_then(|group| group.values.get(&property.value_id))
                .map(|value| {
                    (
                        Table {
                            charge_point: self.url.host_str().unwrap().to_string(),
                            property: property.name.clone(),
                        },
                        Record {
                            timestamp: value.updated.clone(),
                            value: value.value.to_string(),
                        },
                    )
                })
        }))
    }
}
