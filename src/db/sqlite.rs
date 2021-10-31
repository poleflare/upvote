use super::shared::{Adapter, DbAdapter, DbError, Manifest, Status};
use rusqlite::{Connection, Result};
use std::env::current_dir;

pub struct SQLite {
    file: String,
    conn: Option<Connection>,
}

impl SQLite {
    pub fn new(file: &str) -> Self {
        Self {
            file: file.to_string(),
            conn: None,
        }
    }

    /// Set the sqlite's conn.
    pub fn set_conn(&mut self, conn: Option<Connection>) {
        self.conn = conn;
    }
}

impl DbAdapter for SQLite {
    /// Initiate SQLite connection and store the connection in client.
    fn init_db(&mut self, db: &str) -> Result<Status, DbError> {
        let cwd = current_dir().unwrap();
        let target = cwd.to_str().unwrap();
        let db_path = &format!("{}/{}", target, db);
        let conn = match Connection::open(db_path) {
            Ok(conn) => conn,
            Err(err) => {
                eprint!("{:?}", err);

                return Err(DbError {
                    message: "SQLite failed to open file",
                });
            }
        };

        // TODO: debug stdout implementation
        println!("sqlite client connectiong to: {}", db_path);
        self.set_conn(Some(conn));

        Ok(Status::Ok)
    }

    fn get_manifest(&self) -> Manifest {
        Manifest {
            adapter: Adapter::Sqlite,
            table: &self.file,
        }
    }

    fn remove(&self, _force: bool) -> Result<Status, DbError> {
        todo!()
    }
}
