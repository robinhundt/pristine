use crate::db::get_db_pool;
use crate::schemas::{create_schema, Context};
use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{middleware, web, App, HttpServer};
use std::env;

pub mod db;
pub mod handlers;
pub mod schemas;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    let pool = get_db_pool().await;

    let server = HttpServer::new(move || {
        App::new()
            .data(create_schema())
            .data(pool.clone())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::default()
                    .allowed_origin("http://127.0.0.1:8080")
                    .allowed_methods(vec!["POST", "GET"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .service(
                web::resource("/graphql")
                    .route(web::post().to(handlers::graphql_route))
                    .route(web::get().to(handlers::graphql_route)),
            )
            .service(web::resource("/graphiql").route(web::get().to(handlers::graphiql_route)))
    });
    server.bind("127.0.0.1:8080").unwrap().run().await
}
