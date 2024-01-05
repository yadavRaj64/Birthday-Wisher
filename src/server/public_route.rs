use axum::{routing::post, Router};
use sqlx::{Pool, Postgres};

use super::handler::signup;

pub fn public_route() -> Router<Pool<Postgres>> {
    Router::new().route("/signup", post(signup))
    //.route("/login", post(login))
    //.route("/logout", get(logout))
}
