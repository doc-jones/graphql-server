use serde_json::json;

use warp::{Rejection, Reply};
use warp::reply::json;


// check that the server is alive
async fn health_check() -> Result<impl Reply, Rejection> {
    Ok(json(&json!("ok": true)))
}
