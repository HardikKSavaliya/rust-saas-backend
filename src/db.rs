use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use tracing::info;

use crate::migration::Migrator;

pub async fn connect_database(database_url: &str) -> Result<DatabaseConnection, sea_orm::DbErr> {
    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(100)
        .min_connections(5)
        .sqlx_logging(false);

    let db = Database::connect(opt).await?;

    info!("Running database migrations...");
    Migrator::up(&db, None).await?;
    info!("Database migrations complete");

    Ok(db)
}
