use crate::models::Proposal;

use super::sqlite::SQLite;

#[derive(Debug, PartialEq, Eq)]
pub enum Adapter {
    Sqlite,
    Postgres,
    MongoDb,
    DynamoDb,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Status {
    Ok,
    Err,
}

#[derive(Debug)]
pub struct DbError {
    pub message: String,
}

#[derive(Debug)]
pub struct Config<'a> {
    pub adapter: Adapter,
    pub host: &'a String,
}

/// Implements a generic interface between multiple DB-implementations.
pub trait DbAdapter {
    /// Initiate connection to using configured adapter.
    fn init_db(&mut self) -> Result<Status, DbError>;

    /// Returns current Client configuration.
    fn get_config(&self) -> Config;

    /// Removes the all storage for the configured adapter, no data will be saved.
    fn remove(&self) -> Result<Status, DbError>;

    fn get_proposal(&self, title: &str) -> Result<Proposal, DbError>;

    fn insert_proposal(&self, p: Proposal) -> Result<Proposal, DbError>;

    fn delete_proposal(&self, p: Proposal) -> Result<(), DbError>;
}

pub struct Client {}

impl Client {
    /// Creates a new client using given adapter.
    pub fn new(adapter: Adapter, file: &str) -> Box<dyn DbAdapter> {
        let client = match adapter {
            Adapter::Sqlite => SQLite::new(file),
            _ => panic!("db adapter is not implemented"),
        };

        Box::new(client)
    }
}
