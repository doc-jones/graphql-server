mod health_check;

use async_graphql::{MergedObject, SchemaBuilder, EmptyMutation, EmptySubscription, Schema};


#[derive(MergedObject, Default)]
pub struct Query(health_check::Health_CheckQuery);

/// Build GraphQL Schema
pub fn build_schema() -> SchemaBuilder<Query, EmptyMutation, EmptySubscription> {

    Schema::build(Query::default(), EmptyMutation, EmptySubscription)
}