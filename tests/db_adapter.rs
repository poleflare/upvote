use chrono::Utc;
use upvote::{
    db::{
        shared::{DbAdapter, Status},
        Adapter, Client,
    },
    models::Proposal,
};

static DB_LOCATION: &str = "./upvote.db";

#[test]
fn test_sq_lite_adapter() {
    let mut client = init_db_client();
    let init_status = client.init_db().unwrap();
    let config = client.get_config();

    assert_eq!(init_status, Status::Ok);
    assert_eq!(config.adapter, upvote::db::Adapter::Sqlite);
    assert_eq!(config.host, DB_LOCATION);
}

#[test]
fn test_sqlite_insert_proposal() {
    let client = init_db_client();
    let p = Proposal {
        title: String::from("test-proposal-insert"),
        description: String::from("test-description"),
        created: Utc::now(),
        updated: Utc::now(),
        comments: vec![],
        owner: String::from("test-group"),
        upvotes: vec![],
    };

    let p = client.insert_proposal(p).unwrap();

    assert_eq!(p.title, "test-proposal-insert");

    client.delete_proposal(p).unwrap();
}

#[test]
fn test_sqlite_get_proposal() {
    let client = init_db_client();
    let p = generate_test_proposal("get_proposal");
    let p = client.insert_proposal(p).unwrap();
    let p = client.get_proposal(&p.title).unwrap();

    assert_eq!(p.title, "get_proposal");

    client.delete_proposal(p).unwrap();
}

fn init_db_client() -> Box<dyn DbAdapter> {
    let mut client = Client::new(Adapter::Sqlite, DB_LOCATION);
    client.init_db().unwrap();

    client
}

fn generate_test_proposal(name: &str) -> Proposal {
    Proposal {
        title: String::from(name),
        description: String::from("test-description"),
        created: Utc::now(),
        updated: Utc::now(),
        comments: vec![],
        owner: String::from("test-group"),
        upvotes: vec![],
    }
}
