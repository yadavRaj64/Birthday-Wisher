use axum::{routing::post, Router};
use sqlx::{Pool, Postgres};

use super::handler::{login, signup, verify_otp};

pub fn public_route() -> Router<Pool<Postgres>> {
    Router::new().route("/signup", post(signup))
    .route("/login", post(login))
    .route("/verifyOtp", post(verify_otp))
    //.route("/logout", get(logout))
}
