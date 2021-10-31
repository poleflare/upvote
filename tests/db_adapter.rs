use upvote::db::shared::Status;

static DB: &str = "upvote.db";

#[test]
fn test_sq_lite_adapter() {
    let client = upvote::db::Client::new(upvote::db::Adapter::Sqlite, DB);
    let manifest = client.get_manifest();

    assert_eq!(manifest.adapter, upvote::db::Adapter::Sqlite);
    assert_eq!(manifest.table, DB);
}

#[test]
fn test_sqlite_adapter_conn() {
    let mut client = upvote::db::Client::new(upvote::db::Adapter::Sqlite, DB);
    let res = client.init_db(DB).unwrap();

    assert_eq!(res, Status::Ok);
}
