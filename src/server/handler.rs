use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use axum_extra::extract::WithRejection;

use serde::Serialize;
use sqlx::{Pool, Postgres};

use crate::schema::{
    api::{EnteredOtp, LoginUser, NewUser}, friend::{Friend, NewFriend}, otps::Otp, user::User
};

use super::error::{ApiError, FriendError, UserError};

#[derive(Serialize)]
struct Response {
    status: u16,
    message: String,
}

pub async fn signup(
    State(pool): State<Pool<Postgres>>,
    WithRejection(Json(user), _): WithRejection<Json<NewUser>, ApiError>,
) -> impl IntoResponse {
    let result = User::get_user_by_email(&pool, &user.email).await;
    match result {
        Ok(_) => Err(ApiError::BadRequest(
            "User Already Exist with given email id".to_string(),
        )),
        Err(err) => match err {
            UserError::UserNotFound => {
                let result = user.add(&pool).await;
                match result {
                    Ok(user) => {
                        user.send_otp("Signup".to_string(),&pool).await?;
                        Ok(Json(Response {
                        status: StatusCode::OK.as_u16(),
                        message: "User Created".to_string(),
                    }))},
                    Err(_) => Err(ApiError::InternalServerError),
                }
            }
            _ => Err(ApiError::InternalServerError),
        },
    }
}

pub async fn login(
    State(pool): State<Pool<Postgres>>,
    WithRejection(Json(user), _): WithRejection<Json<LoginUser>, ApiError>,
) -> impl IntoResponse {
    let result = User::get_user_by_email(&pool, &user.email).await;
    match result {
        Ok(user) => {
            user.send_otp("Login".to_string(), &pool).await?;
            Ok(Json(Response {
            status: StatusCode::OK.as_u16(),
            message: "User Found".to_string(),
        }))},
        Err(err) => match err {
            UserError::UserNotFound => Err(ApiError::NotFound(
                "User Not Found with given email id".to_string(),
            )),
            _ => Err(ApiError::InternalServerError),
        },
    }
}

pub async fn verify_otp(
    State(pool): State<Pool<Postgres>>,
    WithRejection(Json(entered_otp), _): WithRejection<Json<EnteredOtp>, ApiError>,
) -> impl IntoResponse {
    let result = Otp::get_otp(entered_otp.email, &pool).await;
    match result {
        Ok(mut otp) =>  {
            let result = otp.verify_otp(entered_otp.otp).await;
            if result {
                match otp.otp_used(&pool).await {
                    Ok(_) => {
                        Ok(Json(Response {
                            status: StatusCode::OK.as_u16(),
                            message: "OTP Verified".to_string(),
                        }))
                    },
                    Err(_) => Err(ApiError::InternalServerError),
                }
            }
            else{
                return Err(ApiError::BadRequest(
                    "Invalid OTP".to_string(),
                ))
            }
        },
        Err(err) =>{
            match err {
                sqlx::Error::RowNotFound => Err(ApiError::NotFound(
                    "OTP Not Found with given email id".to_string(),
                )),
                _ => Err(ApiError::InternalServerError),
            }
        },
    }
}

pub async fn show_friends(State(pool): State<Pool<Postgres>>) -> Json<Vec<Friend>> {
    let friends = Friend::get_friends(&pool).await.unwrap();
    Json(friends)
}

pub async fn get_friend(
    Path(id): Path<i32>,
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<Friend>, impl IntoResponse> {
    let friend = Friend::get_friend(&pool, id).await;
    match friend {
        Ok(friend) => Ok(Json(friend)),
        Err(_) => Err((
            StatusCode::NOT_FOUND,
            Json(Response {
                status: StatusCode::NOT_FOUND.as_u16(),
                message: "Friend Not Found with Given Id".to_string(),
            }),
        )),
    }
}

pub async fn remove_friend(
    Path(id): Path<i32>,
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<Friend>, impl IntoResponse> {
    let friend = Friend::get_friend(&pool, id).await;
    match friend {
        Ok(friend) => {
            let friend = friend.remove_friend(&pool).await;
            match friend {
                Ok(friend) => Ok(Json(friend)),
                Err(_) => Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(Response {
                        status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                        message: "Something went wrong".to_string(),
                    }),
                )),
            }
        }
        Err(_) => Err((
            StatusCode::NOT_FOUND,
            Json(Response {
                status: StatusCode::NOT_FOUND.as_u16(),
                message: "Friend Not Found with Given Id".to_string(),
            }),
        )),
    }
}

pub async fn add_friend(
    State(pool): State<Pool<Postgres>>,
    Json(friend): Json<NewFriend>,
) -> Result<Json<Friend>, impl IntoResponse> {
    let result = friend.add(&pool).await;
    match result {
        Ok(friend) => Ok(Json(friend)),
        Err(err) => match err {
            FriendError::FriendAlreadyExist => Err((
                StatusCode::BAD_REQUEST,
                Json(Response {
                    status: StatusCode::BAD_REQUEST.as_u16(),
                    message: "Friend Already Exist with given email id".to_string(),
                }),
            )),

            _ => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Response {
                    status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    message: "Something went wrong".to_string(),
                }),
            )),
        },
    }
}

pub async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(Response {
            status: StatusCode::NOT_FOUND.as_u16(),
            message: "Not Found".to_string(),
        }),
    )
}
