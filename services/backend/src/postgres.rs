use deadpool_postgres::{Config, Pool, Runtime};
use tokio_postgres::NoTls;
use tokio_postgres_migration::Migration;

use crate::error_enums::PostgresError;
use crate::message_structs::{JobRequest, QueueEntriesResponse, QueueEntry};

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
        .expect("could not create postgres pool")
}

pub async fn check_postgres_connection(pool: &Pool) -> Result<bool, PostgresError> {
    let client = pool.get().await?;
    let row = client.query_one("SELECT 1", &[]).await?;
    let result: i32 = row.get(0);

    if result == 1 {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub async fn table_exists(pool: &Pool, table_name: &str) -> Result<bool, PostgresError> {
    let client = pool.get().await?;
    let query = "
        SELECT EXISTS (
            SELECT 1
            FROM information_schema.tables
            WHERE table_schema = 'public'
            AND table_name = $1
        );
    ";
    let row = client.query_one(query, &[&table_name]).await?;
    Ok(row.get(0))
}

pub async fn migrate_up(pool: &Pool) {
    let scripts_up = [(
        "0001_create-queue",
        include_str!("../migrations/0001_create-queue_up.sql"),
    )];

    let mut client = pool
        .get()
        .await
        .expect("could not get postgres client to run migration");
    let migration = Migration::new("migrations".to_string());
    migration
        .up(&mut **client, &scripts_up)
        .await
        .expect("could not run migration");
}

pub async fn add_job_to_queue(
    parsed_request: JobRequest,
    pool: &Pool,
) -> Result<u64, PostgresError> {
    let client = pool.get().await?;
    let query = "
        INSERT INTO queue (input, user_id)
        VALUES ($1, $2)
    ";

    let rows_created: u64 = client
        .execute(query, &[&parsed_request.input, &parsed_request.user])
        .await?;
    Ok(rows_created)
}

pub async fn view_jobs_in_queue(pool: &Pool) -> Result<QueueEntriesResponse, PostgresError> {
    let client = pool.get().await?;
    let query = "SELECT id, input, user_id, being_processed, complete, output FROM queue";
    let rows = client.query(query, &[]).await?;

    let mut entries = Vec::new();
    for row in rows {
        let entry = QueueEntry {
            id: row.get(0),
            input: row.get(1),
            user_id: row.get(2),
            being_processed: row.get(3),
            complete: row.get(4),
            output: row.get(5),
        };
        entries.push(entry);
    }
    Ok(QueueEntriesResponse {
        queue_entries: entries,
    })
}
