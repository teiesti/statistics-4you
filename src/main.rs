mod charge_point;
mod configuration;
mod database;

use {
    crate::{charge_point::ChargePoint, database::Database},
    anyhow::Result,
    configuration::Configuration,
    env_logger::Env,
    futures::future::try_join_all,
    log::{error, info},
    std::time::Duration,
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

    // Establish a connection to the charge points
    let charge_points = try_join_all(
        configuration
            .charge_points
            .iter()
            .map(charge_point::ChargePoint::login),
    )
    .await?;

    // Open the database
    let mut database = Database::open(configuration.database)?;

    // Query the charge points for their status
    let mut interval = tokio::time::interval(Duration::from_secs(configuration.update_interval));
    loop {
        let statuses = try_join_all(charge_points.iter().map(ChargePoint::status)).await?;

        for status in statuses {
            for (table, record) in status {
                database.store(table, record)?;
            }
        }

        interval.tick().await;
    }
}
