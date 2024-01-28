use serde::Deserialize;
use sqlx::PgPool;

use crate::server::error::UserError;

use super::user::User;

#[derive(Default, Clone, Deserialize)]
pub struct NewUser {
    pub name: String,
    pub email: String,
}

impl NewUser {
    pub async fn add(&self, conn: &PgPool) -> Result<User, UserError> {
        let user = User::get_user_by_email(&conn, &self.email).await;
        if user.is_ok() {
            return Err(UserError::UserAlreadyExist);
        } else {
            let result = sqlx::query_as!(
                User,
                "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *",
                self.name,
                self.email
            )
            .fetch_one(conn)
            .await;

            match result {
                Ok(result) => Ok(result),
                Err(err) => Err(UserError::SqlxError(err)),
            }
        }
    }
}

#[derive(Default, Clone, Debug, Deserialize)]
pub struct LoginUser {
    pub email: String,
}
#[derive(Default, Clone, Debug, Deserialize)]
pub struct EnteredOtp {
    pub email: String,
    pub otp: String,
}
