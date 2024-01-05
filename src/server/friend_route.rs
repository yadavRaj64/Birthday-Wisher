use axum::{
    routing::{get, post},
    Router,
};
use sqlx::{Pool, Postgres};

use super::handler::{add_friend, get_friend, remove_friend, show_friends};

pub fn friend_route() -> Router<Pool<Postgres>> {
    Router::new()
        .route("/get_all", get(show_friends))
        .route("/:id", get(get_friend).delete(remove_friend))
        .route("/", post(add_friend))
}
