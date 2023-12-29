use std::io::{self, Write};

use chrono::NaiveDate;
use inquire::validator::Validation;
use inquire::{formatter::DEFAULT_DATE_FORMATTER, CustomType};
use inquire::{min_length, Text};
use regex::Regex;


use crate::schema::InputTypes;


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
                Err(_) => None             }
        },
        InputTypes::Email => {



            let ans = Text::new(prompt)
                .with_validator(val)
                .prompt();
            match ans {
                Ok(value) =>Some(value),
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
                Ok(Validation::Invalid(format!("{} is not a valid email", value).as_str().into()))
            } else {
                Ok(Validation::Valid)
            }
        },
        Err(err) => {
            println!("{:?}",err);
            Ok(Validation::Invalid("Sorry Something went wrong".into()))
        },
    }
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
            Ok(result) => {
                match result {
                    Validation::Valid => assert!(false),
                    Validation::Invalid(_) => assert!(true),
                }
            },
            Err(_) => assert!(false),
        }

        let test2 =val(email2);
        match test2 {
            Ok(result) => {
                match result{
                Validation::Valid => assert!(true),
                    Validation::Invalid(_) => assert!(false),
                }
            }
            Err(_) => assert!(false),
        }
    }

}
