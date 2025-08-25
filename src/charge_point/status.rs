#[derive(Debug)]
pub(crate) struct Status {
    pub(crate) active_charging_scene: DatedValue<ChargingScene>,
}

impl From<super::response::status::Root> for Status {
    fn from(response: super::response::status::Root) -> Self {
        let active_charging_scene =
            &response.configurations_groups["f0d3a233-98d5-4adf-ab67-2ab4a7b7eea4"].values["4f8f981c-f198-49fa-b653-790856129653"];

        Self {
            active_charging_scene: DatedValue {
                value: active_charging_scene.value.clone().into(),
                as_of: active_charging_scene.clone().updated,
            },
        }
    }
}

#[derive(Debug)]
pub(crate) struct DatedValue<T> {
    pub(crate) value: T,
    pub(crate) as_of: String,
}

#[derive(Debug)]
pub(crate) enum ChargingScene {
    Unknown,
    Fast,
    Solar,
    SolarSupported,
    UserDefined,
}

impl From<serde_json::Value> for ChargingScene {
    fn from(value: serde_json::Value) -> Self {
        match value.as_u64() {
            Some(0) => Self::Unknown,
            Some(1) => Self::Fast,
            Some(2) => Self::Solar,
            Some(3) => Self::SolarSupported,
            Some(4) => Self::UserDefined,
            _ => panic!(),
        }
    }
}
