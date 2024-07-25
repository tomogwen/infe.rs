use serde_derive::Serialize;

#[derive(Serialize)]
pub struct HealthCheckResponse {
    pub db_connection: bool,
    pub queue_table_exists: bool,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Serialize)]
pub struct SuccessResponse {
    pub success: bool,
}

#[derive(Serialize)]
pub struct QueueEntry {
    pub id: i32,
    pub input: String,
    pub user_id: String,
    pub being_processed: bool,
    pub complete: bool,
    pub output: String,
}

#[derive(Serialize)]
pub struct QueueEntriesResponse {
    pub queue_entries: Vec<QueueEntry>,
}
