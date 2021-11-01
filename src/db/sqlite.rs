use super::shared::{Adapter, DbAdapter, DbError, Manifest, Status};
use crate::models::Proposal;
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

    fn get_manifest(&self) -> Manifest {
        Manifest {
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

    fn get_proposal(&self, title: &str) -> Result<Proposal, DbError> {
        let mut query = self
            .connection()
            .unwrap()
            .prepare(
                "SELECT 
                title,
                description, 
                created, 
                updated, 
                comments, 
                owner, 
                upvotes 
                FROM proposals WHERE title = (?1)",
            )
            .unwrap();

        let mut res = query
            .query_map([title], |row| {
                let _created: String = row.get(2).unwrap();
                let _updated: String = row.get(3).unwrap();

                Ok(Proposal {
                    title: row.get(0).unwrap(),
                    description: row.get(1).unwrap(),
                    created: _created.parse().unwrap(),
                    updated: _updated.parse().unwrap(),
                    comments: vec![],
                    owner: row.get(6).unwrap(),
                    upvotes: vec![],
                })
            })
            .unwrap();

        res.next().unwrap().or_else(|op| {
            Err(DbError {
                message: format!("{} -> {}", String::from("failed to find proposal"), op),
            })
        })
    }

    fn insert_proposal(&self, p: Proposal) -> Result<Proposal, DbError> {
        let query = self.connection().unwrap().execute(
            "INSERT INTO proposals (
                title, 
                description, 
                created, 
                updated, 
                comments, 
                owner, 
                upvotes
            ) values (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            [
                &p.title,
                &p.description,
                &p.created.to_string(),
                &p.updated.to_string(),
                &p.comments.join(","),
                &p.owner,
                &p.upvotes.join(","),
            ],
        );

        match query {
            Ok(_) => Ok(p),
            Err(err) => {
                eprintln!("{}", err);

                Err(DbError {
                    message: format!("failed to insert {:?}", p),
                })
            }
        }
    }

    fn delete_proposal(&self, p: Proposal) -> Result<(), DbError> {
        self.connection()
            .unwrap()
            .execute("DELETE FROM proposals where (title) = (?1) ", [p.title])
            .unwrap();

        Ok(())
    }
}
