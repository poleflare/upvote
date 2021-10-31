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

type UtcStr = str;

impl Proposal {
    pub fn new(
        title: String,
        description: String,
        created: Option<&UtcStr>,
        updated: Option<&UtcStr>,
        comments: Option<Vec<String>>,
        group: String,
        upvote: Option<Vec<String>>,
    ) -> Self {
        let now = Utc::now();

        Self {
            title,
            description,
            created: match created {
                Some(d) => d.parse().unwrap(),
                None => now,
            },
            updated: match updated {
                Some(d) => d.parse().unwrap(),
                None => now,
            },
            comments: match comments {
                Some(c) => c,
                None => vec![],
            },
            group,
            upvote: match upvote {
                Some(v) => v,
                None => vec![],
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create() {
        let res = Proposal::new(
            "test".to_string(),
            "test-desc".to_string(),
            Some("2020-10-10T21:02:20.346474121Z"),
            Some("2020-10-10T21:02:20.346474121Z"),
            Some(vec!["good stuff".to_string()]),
            "test-grp".to_string(),
            Some(vec!["jane".to_string(), "doe".to_string()]),
        );

        assert_eq!(res.title, "test");
        assert_eq!(res.description, "test-desc");
        assert_eq!(res.group, "test-grp");
        assert_eq!(res.comments.len(), 1);
        assert_eq!(res.upvote.len(), 2);
    }
}
