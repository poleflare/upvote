use super::sqlite::SQLite;
use crate::models::Proposal;

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
}

/// Defines proposal access pattern for each db-client implementation.
pub trait ProposalAdapter {
    /// Retrieves a single proposal that belong to given group.
    fn get_proposal(&self, title: &str, owner: &str) -> Result<Proposal, DbError>;

    /// Fetches all proposals that belong to a given group.
    fn get_proposals(&self, owner: &str) -> Result<Vec<Proposal>, DbError>;

    fn insert_proposal(&self, p: Proposal) -> Result<Proposal, DbError>;

    fn delete_proposal(&self, p: &Proposal) -> Result<(), DbError>;
}

pub struct Client {}

impl Client {
    /// Creates a new client using given adapter.
    pub fn create(adapter: Adapter, file: &str) -> impl DbAdapter + ProposalAdapter {
        match adapter {
            Adapter::Sqlite => SQLite::new(file),
            _ => panic!("db adapter is not implemented"),
        }
    }
}
