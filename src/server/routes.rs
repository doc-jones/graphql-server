use std::convert::Infallible;

use async_graphql::Request;
use serde_json::json;

use crate::schema;

use async_graphql_warp::{Response as GraphQLResponse};
use async_graphql::{Request, Schema};

use warp::{Filter, Rejection, Reply};
use warp::filters::BoxedFilter;
use warp::reply::json;


// check that the server is alive
async fn health_check() -> Result<impl Reply, Rejection> {
    Ok(json(&json!({"ok": true})))
}


pub(super) fn make_routes() -> BoxedFilter<(impl Reply,)> {

    // Build GraphQL schema.
    let schema = schema::build_schema().finish();
    
    
    let health = warp::path::end().and_then(health_check);

    // GraphQL and subscription handler.
    let graphql_handler = warp::post().and(warp::path(graphql).and(
            async_graphql_warp::graphql(schema).and_then(|(schema, request): (Schema<_, _, _>, Request)| async move {
                Ok::<_, Infallible>(GraphQLResponse::from(schema.execute(request).await))
            }
        )
    ));


    health.boxed()
}