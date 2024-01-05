use axum::{routing::{get, post}, Router, Json, extract::{State, Path}, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use sqlx::{Pool, Postgres};
use tower_http::trace::TraceLayer;

use crate::{schema::{Friend, NewFriend, FriendError}, db_connection::establish_connect};

#[derive(Serialize)]
struct Response{
    status: u16,
    message: String,
}

pub async fn serve() {

    let pool = establish_connect().await.unwrap();
    let app = Router::new()
        .route("/friends", get(show_friends))
        .route("/friend/:id", get(get_friend).delete(remove_friend))
        .route("/friend", post(add_friend))
        .with_state(pool)
        .fallback(handler_404)
        .layer(TraceLayer::new_for_http());


    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn show_friends(State(pool): State<Pool<Postgres>>) -> Json<Vec<Friend>> {
    let friends = Friend::get_friends(&pool).await.unwrap();
    Json(friends)
}

async fn get_friend(Path(id): Path<i32>, State(pool): State<Pool<Postgres>>) -> Result<Json<Friend>,impl IntoResponse>{
    let friend = Friend::get_friend(&pool, id).await;
    match friend {
        Ok(friend) => Ok(Json(friend)),
        Err(_) => Err((StatusCode::NOT_FOUND,Json(Response{status:StatusCode::NOT_FOUND.as_u16(),message:"Friend Not Found with Given Id".to_string()}))),
    }
}

async fn remove_friend(Path(id):Path<i32>, State(pool): State<Pool<Postgres>>) -> Result<Json<Friend>,impl IntoResponse>{
    let friend = Friend::get_friend(&pool, id).await;
    match friend {
        Ok(friend) => {
            let friend = friend.remove_friend(&pool).await;
            match friend {
                Ok(friend) => Ok(Json(friend)),
                Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR,Json(Response{status:StatusCode::INTERNAL_SERVER_ERROR.as_u16(),message:"Something went wrong".to_string()}))),
            }
        }
        Err(_) => Err((StatusCode::NOT_FOUND,Json(Response{status:StatusCode::NOT_FOUND.as_u16(),message:"Friend Not Found with Given Id".to_string()}))),
    }
}

async fn add_friend(State(pool): State<Pool<Postgres>>,Json(friend): Json<NewFriend>) -> Result<Json<Friend>,impl IntoResponse>{
    let result = friend.add(&pool).await;
    match result  {
        Ok(friend) => Ok(Json(friend)),
        Err(err) => {
            match err {
                FriendError::FriendAlreadyExist =>
                Err((StatusCode::BAD_REQUEST,Json(Response{status:StatusCode::BAD_REQUEST.as_u16(),message:"Friend Already Exist with given email id".to_string()}))),

                _ =>
                Err((StatusCode::INTERNAL_SERVER_ERROR,Json(Response{status:StatusCode::INTERNAL_SERVER_ERROR.as_u16(),message:"Something went wrong".to_string()}))),
            }
        },
    }
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, Json(Response{status: StatusCode::NOT_FOUND.as_u16(), message: "Not Found".to_string()}))
}
