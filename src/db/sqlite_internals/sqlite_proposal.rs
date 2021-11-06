use rusqlite::Row;

use crate::{
    db::{
        shared::{DbError, ProposalAdapter},
        sqlite::SQLite,
    },
    models::Proposal,
};

impl ProposalAdapter for SQLite {
    fn get_proposal(&self, title: &str, owner: &str) -> Result<Proposal, DbError> {
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
                FROM proposals WHERE title = (?1) AND owner = (?2)",
            )
            .unwrap();
        let mut res = query.query_map([title, owner], db_row_to_proposal).unwrap();

        res.next().unwrap().map_err(|op| DbError {
            message: format!("{} -> {}", String::from("failed to find proposal"), op),
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

    fn delete_proposal(&self, p: &Proposal) -> Result<(), DbError> {
        self.connection()
            .unwrap()
            .execute("DELETE FROM proposals where (title) = (?1) ", [&p.title])
            .unwrap();

        Ok(())
    }

    fn get_proposals(&self, owner: &str) -> Result<Vec<Proposal>, DbError> {
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
                FROM proposals WHERE owner = (?1)",
            )
            .unwrap();
        let res = query.query_map([owner], db_row_to_proposal).unwrap();
        let mut result_list: Vec<Proposal> = vec![];

        res.for_each(|r| {
            if let Ok(p) = r {
                result_list.push(p)
            }
        });

        Ok(result_list)
    }
}

fn db_row_to_proposal<E>(row: &Row) -> Result<Proposal, E> {
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
}
