use askama::Template;
use lettre::transport::smtp::Error as SmtpError;
use rand::Rng;
use sqlx::{Error as SqlxError, PgPool};

use crate::helper::utils::send_email;

pub struct Otp {
    email: String,
    otp: String,
    created_for: String,
    used: bool,
    sent: bool,
}

#[derive(Template)]
#[template(path = "otp.html")]
pub struct OtpTemp<'a> {
    pub(crate) otp: &'a str,
    pub(crate) used_for: &'a str,
}
impl Otp {
    pub async fn new(
        email: String,
        used_for: String,
        transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<Self, SqlxError> {
        let otp = Self::gen_otp().to_string();
        let otp = Self {
            email,
            otp,
            created_for: used_for,
            used: false,
            sent: false,
        };

        sqlx::query_as!(
            Otp,
            "INSERT INTO otps (email, otp, created_for, used, sent) VALUES ($1, $2, $3, $4, $5) RETURNING *",
            otp.email,
            otp.otp,
            otp.created_for,
            otp.used,
            otp.sent
        )
        .fetch_one(&mut **transaction)
        .await
    }

    fn gen_otp() -> i32 {
        rand::thread_rng().gen_range(1000..10000)
    }

    pub fn get_opt_template(&self) -> OtpTemp {
        OtpTemp {
            otp: &self.otp,
            used_for: &self.created_for,
        }
    }

    pub async fn send_otp(&mut self) -> Result<(), SmtpError> {
        let body = self.get_opt_template().render().unwrap();
        send_email(&self.email, "otp".to_string(), body.to_string())
            .await
            .unwrap();
        Ok(())
    }

    pub async fn verify_otp(&mut self, otp: String) -> bool {
        if self.otp == otp {
            return true;
        }
        false
    }

    pub async fn otp_sent(
        &mut self,
        transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<(), SqlxError> {
        sqlx::query!(
            "UPDATE otps SET sent = $1 WHERE email = $2 AND otp = $3",
            true,
            self.email,
            self.otp
        )
        .execute(&mut **transaction)
        .await?;
        Ok(())
    }

    pub async fn otp_used(
        &mut self,
        pool: &PgPool,
    ) -> Result<(), SqlxError> {
        sqlx::query!(
            "DELETE FROM otps WHERE email = $1 AND otp = $2",
            self.email,
            self.otp
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn get_otp(email: String, pool: &PgPool) -> Result<Otp, SqlxError> {
        let result = sqlx::query_as!(
            Otp,
            "SELECT * FROM otps WHERE email = $1 ", email)
            .fetch_one(pool)
            .await;
        match result {
            Ok(result) => Ok(result),
            Err(err) => match err {
                sqlx::Error::RowNotFound => Err(err),
                _ => Err(err),
            },
        }
    }
}
