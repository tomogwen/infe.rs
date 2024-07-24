use deadpool_postgres::{Config, Pool, Runtime};
use tokio_postgres::NoTls;
use tokio_postgres_migration::Migration;

fn create_config() -> Config {
    let mut cfg = Config::new();
    if let Ok(host) = std::env::var("PG_HOST") {
        cfg.host = Some(host);
    }
    if let Ok(dbname) = std::env::var("PG_DBNAME") {
        cfg.dbname = Some(dbname);
    }
    if let Ok(user) = std::env::var("PG_USER") {
        cfg.user = Some(user);
    }
    if let Ok(password) = std::env::var("PG_PASSWORD") {
        cfg.password = Some(password);
    }
    cfg
}

pub fn create_pool() -> Pool {
    create_config()
        .create_pool(Some(Runtime::Tokio1), NoTls)
        .expect("couldn't create postgres pool")
}

pub async fn table_exists(pool: &Pool, table_name: &str) -> bool {
    let client = pool.get().await.expect("couldn't get postgres client");
    let query = "
        SELECT EXISTS (
            SELECT 1
            FROM information_schema.tables
            WHERE table_schema = 'public'
            AND table_name = $1
        );
    ";

    match client.query_one(query, &[&table_name]).await {
        Ok(row) => row.get(0),
        Err(_) => false,
    }
}

pub async fn migrate_up(pool: &Pool) {
    let scripts_up = [(
        "0001_create-queue",
        include_str!("../migrations/0001_create-queue_up.sql"),
    )];

    let mut client = pool.get().await.expect("couldn't get postgres client");
    let migration = Migration::new("migrations".to_string());
    migration
        .up(&mut **client, &scripts_up)
        .await
        .expect("couldn't run migrations");
}
