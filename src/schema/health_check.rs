use async_graphql::Object;

#[derive(Default)]
pub struct HealthCheckQuery;

#[Object]
impl HealthCheckQuery {
    /// Return 'true' if the GraphQL server is available.
    async fn health_check(&self) -> bool {
        true
    }
}