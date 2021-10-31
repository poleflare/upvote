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
pub struct DbError<'a> {
    pub message: &'a str,
}

#[derive(Debug)]
pub struct Manifest<'a> {
    pub adapter: Adapter,
    pub table: &'a String,
}

/// Implements a generic interface between multiple DB-implementations.
pub trait DbAdapter {
    /// Initiate connection to using configured adapter.
    fn init_db(&mut self, table_name: &str) -> Result<Status, DbError>;

    /// Returns current Client configuration.
    fn get_manifest(&self) -> Manifest;

    /// Removes the all storage for the configured adapter, no data will be saved.
    fn remove(&self, force: bool) -> Result<Status, DbError>;
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
