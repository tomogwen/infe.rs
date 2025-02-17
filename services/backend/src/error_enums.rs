use deadpool_postgres::PoolError;
use thiserror::Error;
use tokio_postgres::Error as TokioError;

#[derive(Error, Debug)]
pub enum PostgresError {
    #[error("could not allocate client from pool")]
    Pool(#[from] PoolError),
    #[error("postgres query failed")]
    Query(#[from] TokioError),
}
