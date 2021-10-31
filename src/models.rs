use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Proposal {
    pub title: String,
    pub description: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub comments: Vec<String>, // comment.id(s)
    pub group: String,         // group.id
    pub upvote: Vec<String>,   // User.id(s)
}

impl Proposal {
    pub fn new() -> Self {
        Self {
            title: "test-title".to_string(),
            description: "test-description".to_string(),
            created: Utc::now(),
            updated: Utc::now(),
            comments: vec![],
            group: "test-group".to_string(),
            upvote: vec![],
        }
    }
}

pub struct User {
    pub id: String,
}

pub struct Group {
    pub id: String,
}

pub struct Comment {
    pub user: String,
    pub body: String,
}
