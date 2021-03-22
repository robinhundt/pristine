use actix_web::{web, Error, HttpResponse};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

use crate::schemas::{create_schema, Context, Schema};
use sqlx::SqlitePool;

pub async fn graphiql_route() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(graphiql_source("/graphql", None))
}

pub async fn graphql_route(
    schema: web::Data<Schema>,
    pool: web::Data<SqlitePool>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let context = Context {
        pool: pool.get_ref().to_owned(),
    };
    let res = {
        let res = data.execute(&schema, &context).await;
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    }
    .map_err(Error::from)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(res))
}
