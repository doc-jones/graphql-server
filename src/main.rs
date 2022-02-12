mod server;

extern crate log;
extern crate pretty_env_logger;

#[tokio::main]
async fn main() {
    // Initialize logger
    pretty_env_logger::init();
    
    // Start server to handle requests.
    server::start(addr: ([127, 0, 0, 1], 9090)).await;
}
