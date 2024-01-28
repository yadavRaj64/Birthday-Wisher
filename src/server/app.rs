use axum::Router;

use tower_http::trace::TraceLayer;
use tracing::info;

use crate::{
    helper::db_connection::establish_connect, server::{friend_route::friend_route, handler::handler_404, public_route::public_route}
};

pub async fn serve() {
    let pool = establish_connect().await.unwrap();
    let app = Router::new()
        .nest("/friend", friend_route())
        .nest("/", public_route())
        .with_state(pool)
        .fallback(handler_404)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    info!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
