user std::net::SocketAddr;

mod routes;

/// Start the API server
async fn start(addr: impl Into<SocketAddr>) {
    warp::serve(routes::make_routes()).run(addr).await;
}