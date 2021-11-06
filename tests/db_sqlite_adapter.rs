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

    client.delete_proposal(&p).unwrap();
}

#[test]
fn test_sqlite_get_proposal() {
    let client = init_db_client();
    let p = generate_test_proposal("get_proposal");
    let p = client.insert_proposal(p).unwrap();
    let p = client.get_proposal(&p.title, "test-group").unwrap();

    assert_eq!(p.title, "get_proposal");

    client.delete_proposal(&p).unwrap();
}

#[test]
fn test_sqlite_get_proposals() {
    let client = init_db_client();
    let p1 = generate_test_proposal("get_proposals_1");
    let p2 = generate_test_proposal("get_proposals_2");

    client.insert_proposal(p1).unwrap();
    client.insert_proposal(p2).unwrap();

    let res = client.get_proposals("test-group").unwrap();

    assert_eq!(res[0].title, "get_proposals_1");
    assert_eq!(res[1].title, "get_proposals_2");

    client.delete_proposal(&res[0]).unwrap();
    client.delete_proposal(&res[1]).unwrap();
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
