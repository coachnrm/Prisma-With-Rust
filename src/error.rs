use prisma_client_rust::QueryError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Query error")]
    Query(#[from] QueryError)
}