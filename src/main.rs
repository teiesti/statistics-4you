mod charge_point;
mod command_line;
mod configuration;

use {
    anyhow::Result,
    configuration::Configuration,
    env_logger::Env,
    log::{error, info},
};

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_COMMIT: &str = env!("PKG_COMMIT");

#[tokio::main]
async fn main() {
    if let Err(err) = try_main().await {
        error!("{:?}", err);
    }
}

async fn try_main() -> Result<()> {
    // Initialize logging
    let env = Env::default().default_filter_or(format!("{}=info", PKG_NAME.replace("-", "_")));
    env_logger::init_from_env(env);

    // Log the version
    info!("Starting {} {} ({})", PKG_NAME, PKG_VERSION, PKG_COMMIT);

    // Load the configuration
    let configuration = Configuration::discover()?;

    // TODO

    Ok(())
}
