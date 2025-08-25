use {
    anyhow::{Context, Result, bail},
    const_format::formatcp,
    log::{debug, info, trace},
    serde::Deserialize,
    std::{env, fs::read_to_string, ops::Deref, path::Path},
};

const SEARCH_PATHS: &[&str] = &[
    "./",
    formatcp!("~/.config/{}/", crate::PKG_NAME),
    formatcp!("/etc/{}/", crate::PKG_NAME),
];
const FILE_NAME: &str = formatcp!("{}.json", crate::PKG_NAME);

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Configuration {
    pub(crate) charge_points: Vec<ChargePoint>,
}

impl Configuration {
    pub(crate) fn discover() -> Result<Self> {
        info!("Searching for a configuration file");

        let manifest_dir = env::var("CARGO_MANIFEST_DIR");
        let mut paths = manifest_dir
            .iter()
            .map(String::as_str)
            .chain(SEARCH_PATHS.iter().map(Deref::deref))
            .map(Path::new)
            .map(|directory| directory.join(FILE_NAME));

        let path = loop {
            match paths.next() {
                Some(path) => {
                    trace!("Trying {}", path.display());
                    if path.exists() {
                        break path;
                    }
                }
                None => bail!("Could not find a configuration file"),
            }
        };

        Self::load(path)
    }

    fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        info!("Loading configuration from {}", path.display());

        let string =
            read_to_string(path).with_context(|| format!("Could not read {}", path.display()))?;

        let config = serde_json::from_str(&string)
            .with_context(|| format!("Could not decode {}", path.display()))?;

        debug!("Here's your configuration:\n{:#?}", config);

        Ok(config)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct ChargePoint {
    pub(crate) url: String,
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) observe: Vec<Property>,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Property {
    pub(crate) name: String,
    pub(crate) group_id: String,
    pub(crate) value_id: String,
}
