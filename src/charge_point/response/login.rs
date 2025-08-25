use serde::Deserialize;

#[derive(Debug, Deserialize)]

pub(crate) struct Root {
    pub(crate) token: String,
}
