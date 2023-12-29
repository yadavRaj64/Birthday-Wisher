use crate::schema::{BirthdayWisher, Friends, InputTypes};
use crate::utils::{clear, get_text_input};
use dotenvy::dotenv;
use inquire::Select;
use lettre::Transport;
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, Message,
    SmtpTransport,
};
use std::env;
use std::fs;

pub async fn send(to: String, subject: String, body: String) {
    dotenv().ok();
    let smtp_username =
        env::var("SMTP_USERNAME").expect("Please set up SMTP_USERNAME in your environment");
    let from_email = format!("Rahul <{}>", smtp_username.as_str());
    let smtp_password =
        env::var("SMTP_PASSWORD").expect("Please set up SMTP_PASSWORD in your environment");
    let smtp_host = env::var("SMTP_HOST").expect("Please set up SMTP_HOST in your environment");
    let body_content = fs::read_to_string(body).expect("Invalid path!");

    let email = Message::builder()
        .from(from_email.as_str().parse().unwrap())
        .to(to.parse().unwrap())
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(body_content)
        .unwrap();

    let creds = Credentials::new(smtp_username.to_owned(), smtp_password.to_owned());

    let mailer = SmtpTransport::relay(smtp_host.as_str())
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }
}

pub async fn start() {
    let mut friends = Friends::new();
    clear();
    loop {
        let choice = Select::new(
            "Hello, How can I help you?",
            BirthdayWisher::OPTIONS.to_vec(),
        )
        .prompt();
        match choice {
            Ok(value) => match value {
                BirthdayWisher::AddFriend => {
                    let friend = Friends::get_friend_info();
                    match friend {
                        Some(friend) => {
                            friends.add(friend).await;
                        }
                        None => println!("Sorry!, Something want wrong"),
                    }
                }
                BirthdayWisher::RemoveFriend => {
                    let id = get_text_input("Enter id of your friend", InputTypes::Num);
                    match id {
                        Some(id) => {
                            friends.remove(id.parse().unwrap()).await;
                        }
                        None => println!("Invalid id!!!!!!!!!!"),
                    }
                }
                BirthdayWisher::ShowFriends => {
                    friends.show_friends().await;
                }
                BirthdayWisher::Exit => break,
                BirthdayWisher::ClearScreen => clear(),
            },
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }
}
