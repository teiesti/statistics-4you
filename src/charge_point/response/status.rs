#![allow(dead_code)]

use {serde::Deserialize, std::collections::HashMap};

#[derive(Debug, Deserialize)]
pub(crate) struct Root {
    #[serde(rename = "accessLevels")]
    pub(crate) access_levels: serde_json::Value, // TODO: Replace with actual type

    #[serde(
        rename = "configurationsGroups",
        deserialize_with = "Root::deserialize_configurations_groups"
    )]
    pub(crate) configurations_groups: HashMap<String, ConfigurationsGroup>,
}

impl Root {
    fn deserialize_configurations_groups<'de, D>(
        deserializer: D,
    ) -> Result<std::collections::HashMap<String, ConfigurationsGroup>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let vec = Vec::<ConfigurationsGroup>::deserialize(deserializer)?;
        Ok(vec
            .into_iter()
            .map(|group| (group.id.to_lowercase(), group))
            .collect())
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct ConfigurationsGroup {
    pub(crate) id: String,

    pub(crate) label: String,

    #[serde(rename = "isMandatory")]
    pub(crate) is_mandatory: bool,

    #[serde(rename = "internalConfigurationElements")]
    pub(crate) internal_configuration_elements: Vec<serde_json::Value>, // TODO: Replace with actual type

    #[serde(deserialize_with = "ConfigurationsGroup::deserialize_values")]
    pub(crate) values: HashMap<String, Value>,

    #[serde(rename = "webInterfaceAction")]
    pub(crate) web_interface_action: serde_json::Value, // TODO: Replace with actual type

    #[serde(rename = "accessLevels")]
    pub(crate) access_levels: serde_json::Value, // TODO: Replace with actual type
}

impl ConfigurationsGroup {
    fn deserialize_values<'de, D>(
        deserializer: D,
    ) -> Result<std::collections::HashMap<String, Value>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let vec = Vec::<Value>::deserialize(deserializer)?;
        Ok(vec
            .into_iter()
            .map(|value| (value.id.to_lowercase(), value))
            .collect())
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Value {
    pub(crate) id: String,

    pub(crate) label: String,

    pub(crate) r#type: String,

    pub(crate) value: serde_json::Value,

    pub(crate) updated: String,

    pub(crate) access: String,

    #[serde(rename = "isRebootRequired")]
    pub(crate) is_reboot_required: Option<bool>,

    #[serde(rename = "isExpertValue")]
    pub(crate) is_expert_value: bool,

    #[serde(rename = "webInterfaceValidators")]
    pub(crate) web_interface_validators: Option<serde_json::Value>, // TODO: Replace with actual type

    #[serde(rename = "infoText")]
    pub(crate) info_text: Option<String>,
}
