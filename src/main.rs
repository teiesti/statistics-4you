mod charge_point;
mod configuration;
mod database;

use {
    crate::database::Database,
    anyhow::Result,
    configuration::Configuration,
    env_logger::Env,
    log::{error, info},
    std::{path::Path, time::Duration},
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

    // Log in to the charge points
    let charge_points = futures::future::try_join_all(
        configuration
            .charge_points
            .iter()
            .map(charge_point::ChargePoint::login),
    )
    .await?;

    // TODO: Make this beautiful, use futures, add configuration options and logging
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    loop {
        let statuses = futures::future::try_join_all(
            charge_points.iter().map(charge_point::ChargePoint::status),
        )
        .await?;

        let mut database = Database::open(Path::new("./data").to_path_buf()).unwrap();

        for status in statuses {
            for (table, record) in status.into_iter() {
                database.store(table, record).await?;
            }
        }

        interval.tick().await;
    }
}
