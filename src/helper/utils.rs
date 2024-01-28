use std::env;
use std::io::{self, Write};

use chrono::NaiveDate;
use inquire::validator::Validation;
use inquire::{formatter::DEFAULT_DATE_FORMATTER, CustomType};
use inquire::{min_length, Text};
use lettre::transport::stub::Error;
use lettre::Transport;
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, Message,
    SmtpTransport,
};
use regex::Regex;

use crate::schema::friend::InputTypes;

pub fn get_text_input(prompt: &str, input_type: InputTypes) -> Option<String> {
    match input_type {
        InputTypes::Text => {
            let ans = Text::new(prompt)
                .with_validator(min_length!(2, "Minimum 2 letters are required"))
                .prompt();
            match ans {
                Ok(string) => Some(string),
                Err(_) => None,
            }
        }
        InputTypes::Date => {
            let date = CustomType::<NaiveDate>::new(prompt)
                .with_placeholder("dd/mm/yyyy")
                .with_parser(&|i| NaiveDate::parse_from_str(i, "%d/%m/%Y").map_err(|_e| ()))
                .with_formatter(DEFAULT_DATE_FORMATTER)
                .with_error_message("Please type a valid date.")
                .prompt();
            match date {
                Ok(value) => Some(value.to_string()),
                Err(_) => None,
            }
        }
        InputTypes::Num => {
            let ans = Text::new(prompt)
                .with_validator(min_length!(1, "Minimum 1 letters are required"))
                .prompt();
            match ans {
                Ok(value) => Some(value.to_string()),
                Err(_) => None,
            }
        }
        InputTypes::Email => {
            let ans = Text::new(prompt).with_validator(val).prompt();
            match ans {
                Ok(value) => Some(value),
                Err(_) => None,
            }
        }
    }
}

pub fn clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    io::stdout().flush().unwrap();
}

fn val(value: &str) -> Result<Validation, Box<dyn std::error::Error + Send + Sync>> {
    let reg = Regex::new(r"^[\w\.]+@([\w]+\.)+[\w-]{2,4}$");
    match reg {
        Ok(reg) => {
            if !reg.is_match(value) {
                Ok(Validation::Invalid(
                    format!("{} is not a valid email", value).as_str().into(),
                ))
            } else {
                Ok(Validation::Valid)
            }
        }
        Err(err) => {
            println!("{:?}", err);
            Ok(Validation::Invalid("Sorry Something went wrong".into()))
        }
    }
}

pub async fn send_email(to: &str, subject: String, body: String) -> Result<(), Error> {
    let smtp_username =
        env::var("SMTP_USERNAME").expect("Please set up SMTP_USERNAME in your environment");
    let from_email = format!("Rahul <{}>", smtp_username.as_str());
    let smtp_password =
        env::var("SMTP_PASSWORD").expect("Please set up SMTP_PASSWORD in your environment");
    let smtp_host = env::var("SMTP_HOST").expect("Please set up SMTP_HOST in your environment");

    let email = Message::builder()
        .from(from_email.as_str().parse().unwrap())
        .to(to.parse().unwrap())
        .subject(subject)
        .header(ContentType::TEXT_HTML)
        .body(body)
        .unwrap();

    let creds = Credentials::new(smtp_username.to_owned(), smtp_password.to_owned());

    let mailer = SmtpTransport::relay(smtp_host.as_str())
        .unwrap()
        .credentials(creds)
        .build();

    mailer.send(&email).unwrap();
    Ok(())
}

#[cfg(test)]
mod tests {
    use inquire::validator::Validation;

    use super::val;

    #[test]
    fn test_email_validator() {
        let email1 = "testemial.com";
        let email2 = "test@email.com";

        let test1 = val(email1);
        match test1 {
            Ok(result) => match result {
                Validation::Valid => assert!(false),
                Validation::Invalid(_) => assert!(true),
            },
            Err(_) => assert!(false),
        }

        let test2 = val(email2);
        match test2 {
            Ok(result) => match result {
                Validation::Valid => assert!(true),
                Validation::Invalid(_) => assert!(false),
            },
            Err(_) => assert!(false),
        }
    }
}
