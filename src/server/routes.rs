use std::convert::Infallible;

use serde_json::json;

use crate::schema;

use warp::{filters::BoxedFilter, Filter, Rejection, Reply, http::Response, reply::json};

use async_graphql::{Request, Schema, http::{playground_source, GraphQLPlaygroundConfig}};
use async_graphql_warp::{Response as GraphQLResponse};





// check that the server is alive
async fn health_check() -> Result<impl Reply, Rejection> {
    Ok(json(&json!({"ok": true})))
}


pub(super) fn make_routes() -> BoxedFilter<(impl Reply,)> {

    // Build GraphQL schema.
    let schema = schema::build_schema().finish();
    
    
    let health = warp::path::end().and_then(health_check);

    // GraphQL and subscription handler.
    let graphql_handler = warp::post().and(warp::path("graphql").and(
            async_graphql_warp::graphql(schema).and_then(|(schema, request): (Schema<_, _, _>, Request)| async move {
                Ok::<_, Infallible>(GraphQLResponse::from(schema.execute(request).await))
            }
        )
    ));

    // GraphQL
    let graphql_playground = warp::path("playground").map(|| {
        Response::builder()
            .header("cotent-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
    });

    // Combine all routes.
    health
        .or(graphql_handler)
        .or(graphql_playground)
        .boxed()
}