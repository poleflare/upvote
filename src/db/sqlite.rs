use super::shared::{Adapter, Config, DbAdapter, DbError, Status};
use rusqlite::{Connection, Result};
use std::env::current_dir;
use std::fs::remove_file;

pub struct SQLite {
    file: String,
    connection: Option<Connection>,
}

impl SQLite {
    pub fn new(file: &str) -> Self {
        Self {
            file: file.to_string(),
            connection: None,
        }
    }

    pub fn file(&self) -> &String {
        &self.file
    }

    /// Set the sqlite's connection handle.
    pub fn set_connection(&mut self, conn: Option<Connection>) {
        self.connection = conn;
    }

    /// Get a reference to the sqlite's connection handle.
    pub fn connection(&self) -> Option<&Connection> {
        self.connection.as_ref()
    }
}

impl DbAdapter for SQLite {
    /// Initiate SQLite connection and store the connection in client.
    fn init_db(&mut self) -> Result<Status, DbError> {
        let cwd = current_dir().unwrap();
        let target = cwd.to_str().unwrap();
        let db_path = &format!("{}/{}", target, self.file);
        let conn = match Connection::open(db_path) {
            Ok(conn) => conn,
            Err(err) => {
                eprint!("{:?}", err);

                return Err(DbError {
                    message: String::from("SQLite failed to open file"),
                });
            }
        };

        self.set_connection(Some(conn));
        self.connection()
            .unwrap()
            .execute(
                "create table if not exists manifest (adapter text primary key)",
                [],
            )
            .unwrap();
        self.connection()
            .unwrap()
            .execute(
                "create table if not exists proposals (
                id integer primary key,
				title text not null unique, 
				description text not null,
				created text not null,
				updated text not null,
				comments text not null,
				owner text not null,
                upvotes text not null)",
                [],
            )
            .unwrap();

        // TODO: debug stdout implementation
        println!("sqlite client connectiong to: {}", db_path);

        Ok(Status::Ok)
    }

    fn get_config(&self) -> Config {
        Config {
            adapter: Adapter::Sqlite,
            host: &self.file,
        }
    }

    fn remove(&self) -> Result<Status, DbError> {
        match remove_file(self.file.as_str()) {
            Ok(_) => Ok(Status::Ok),
            Err(_) => Err(DbError {
                message: String::from("failed to remove sqlite db file"),
            }),
        }
    }
}
