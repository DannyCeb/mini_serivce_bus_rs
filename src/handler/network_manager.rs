use axum::{routing::post, Router};

use crate::entities::queue::handler_push;


pub fn routes_queue() -> Router {
    Router::new()
        .route("/push", post(handler_push))
        //.route("/pop", post(handler_luhn))
}

