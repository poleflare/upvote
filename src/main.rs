use std::net::TcpListener;

use upvote::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")
        .expect("Failed to bind port");

    let server = run(listener).await?;

    server.await
}