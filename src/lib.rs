use actix_web::dev::Server;
use actix_web::{guard, web, App, HttpResponse, HttpServer, Responder, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql_actix_web::{Request, Response};
use std::net::TcpListener;

pub mod db;
pub mod models;

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn health(&self) -> bool {
        true
    }
}

type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

async fn graphql(schema: web::Data<AppSchema>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> Result<HttpResponse> {
    let source = playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"));
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}

pub async fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let schema: AppSchema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(graphql))
            .service(
                web::resource("/")
                    .guard(guard::Get())
                    .to(graphql_playground),
            )
            .route("/health_check", web::get().to(health_check))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
