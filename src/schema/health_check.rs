use async_graphql::Object;

#[derive(Default)]
pub struct Health_CheckQuery;

#[Object]
impl Health_CheckQuery {
    /// Return 'true' if the GraphQL server is available.
    async fn health_check(&self) -> bool {
        true
    }
}