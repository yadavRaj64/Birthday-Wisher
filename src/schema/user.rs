use serde:: Serialize;
use sqlx::{Error, PgPool};

use crate::server::error::{ApiError, UserError};

use super::otps::Otp;

#[derive(Default, Clone, Debug, Serialize)]
pub struct User {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) email: String,
}

impl User {
    pub async fn get_user_by_email(conn: &PgPool, email: &str) -> Result<User, UserError> {
        let result = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1 ", email)
            .fetch_one(conn)
            .await;
        match result {
            Ok(result) => Ok(result),
            Err(err) => match err {
                Error::RowNotFound => Err(UserError::UserNotFound),
                _ => Err(UserError::SqlxError(err)),
            },
        }
    }

    pub async fn send_otp(self, used_for: String, conn: &PgPool) -> Result<(), ApiError> {
        let transaction_result = conn.begin().await;
        if let Ok(mut transaction_ok) = transaction_result {
            let otp = Otp::new(self.email, used_for, &mut transaction_ok).await;
            if let Ok(mut otp_ok) = otp {
                match otp_ok.send_otp().await {
                    Ok(_) => {
                        match otp_ok.otp_sent(&mut transaction_ok).await {
                            Ok(_) => match transaction_ok.commit().await {
                                Ok(_) => return Ok(()),
                                Err(_) => {
                                    return Err(ApiError::TransactionError(
                                        "Failed to commit transaction".to_string(),
                                    ))
                                }
                            }, // otp sent ok
                            Err(_) => return Err(ApiError::InternalServerError),
                        } // otp sent
                    } // send otp ok
                    Err(_) => match transaction_ok.rollback().await {
                        Ok(_) => return Err(ApiError::EmailError),
                        Err(_) => {
                            return Err(ApiError::TransactionError(
                                "Failed to rollback transaction".to_string(),
                            ))
                        }
                    },
                } // send otp
            }
            // otp ok
            else {
                match transaction_ok.rollback().await {
                    Ok(_) => return Err(ApiError::InternalServerError),
                    Err(_) => {
                        return Err(ApiError::TransactionError(
                            "Failed to rollback transaction".to_string(),
                        ))
                    }
                }
            } // otp not ok
        } else {
            return Err(ApiError::TransactionError(
                "Failed to start transaction".to_string(),
            ));
        }
    }
}
