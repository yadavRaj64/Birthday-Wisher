use std::{
    fmt::{Display, Formatter},
    str::FromStr
};

use crate::{helper::{db_connection::establish_connect, utils::{get_text_input, send_email}}, server::error::FriendError};

use askama::Template;
use chrono::NaiveDate;
use inquire::Confirm;

use serde::{Serialize, Deserialize};
use tabled::{Table, Tabled};

use sqlx::{Error, PgPool};


#[derive(Template)]
#[template(path= "index.html")]
struct BirthdayTemp<'a> {
    name: &'a str,
}


#[derive(Default, Tabled, Clone, Debug, Serialize)]
pub struct Friend {
    id: i32,
    name: String,
    email: String,
    dob: NaiveDate,
}


impl Friend {
    // get_friend is used to get friend from the database table
    // It takes two argument
    // 1. conn: &PgPool (Postgres connection) >> It is used to connect with database
    // 2. id: i32 >> It is used to get friend with provided id
    pub async fn get_friend(conn: &PgPool, id: i32) -> Result<Friend, FriendError> {
        let result = sqlx::query_as!(Friend, "SELECT * FROM friend WHERE id = ($1)", id)
            .fetch_one(conn)
            .await;
        match result {
            Ok(result) => Ok(result),
            Err(err) => {
                match err {
                    Error::RowNotFound => Err(FriendError::FriendNotFound),
                    _ => Err(FriendError::SqlxError(err)),
                }
            }
        }
    }

    // get_friends is used to get all friends detail from the database table
    pub async fn get_friends(conn: &PgPool) -> Result<Vec<Friend>, Error> {
        let friends = sqlx::query_as!(Friend, "SELECT * FROM friend",)
            .fetch_all(conn)
            .await;
        friends
    }

    // remove_friend is used to remove friend from the database table
    pub async fn remove_friend(self, conn: &PgPool) -> Result<Friend, FriendError> {
        let friend = sqlx::query_as!(
            Friend,
            "DELETE FROM friend WHERE id = ($1) RETURNING *",
            self.id
        )
        .fetch_one(conn)
        .await;
        match friend {
            Ok(friend) => Ok(friend),
            Err(err) => {
                match err {
                    Error::RowNotFound => Err(FriendError::FriendNotFound),
                    _ => Err(FriendError::SqlxError(err)),
                }
            }
        }
    }

    pub async fn send_birthday_email(&self){
        let subject = format!("Happy Birthday {}!",self.name);
        // let body = format!("Happy Birthday {}!", self.name);
        let body = BirthdayTemp{name: &self.name};
        send_email(&self.email, subject, body.render().unwrap()).await.unwrap();
    }

    async fn get_friend_by_email(conn : &PgPool, email : &str) -> Result<Friend, Error>{
        let result= sqlx::query_as!(
            Friend,
            "SELECT * FROM friend WHERE email = $1 ",
            email
        )
        .fetch_one(conn)
        .await;
        result
    }

}

#[derive(Default, Tabled, Clone, Deserialize)]
pub struct NewFriend {
    name: String,
    email: String,
    dob: NaiveDate,
}

impl NewFriend {
    pub async fn add(&self, conn: &PgPool) -> Result<Friend, FriendError> {
        let friend = Friend::get_friend_by_email(&conn, &self.email).await;
        if friend.is_ok() {
            return Err(FriendError::FriendAlreadyExist);
        }

        let result = sqlx::query_as!(
            Friend,
            "INSERT INTO friend (name, email, dob) VALUES($1, $2, $3)  RETURNING *",
            self.name,
            self.email,
            self.dob
        )
        .fetch_one(conn)
        .await;

        match result {
            Ok(result) =>  Ok(result),
            Err(err) => Err(FriendError::SqlxError(err))
        }
    }
}

pub enum InputTypes {
    Text,
    Date,
    Num,
    Email,
}

#[derive(Clone)]
pub struct Friends {
    pub friends: Vec<Friend>,
}

impl Friends {
    pub fn new() -> Friends {
        Friends {
            friends: Vec::new(),
        }
    }

    pub fn get_friend_info() -> Option<NewFriend> {
        let mut friend = NewFriend::default();

        if let Some(name) = get_text_input("Enter you Friend's Name", InputTypes::Text) {
            friend.name = name;
        } else {
            return None;
        }

        if let Some(dob) = get_text_input("Enter you Friend's DOB", InputTypes::Date) {
            friend.dob = NaiveDate::from_str(dob.as_str()).unwrap();
        } else {
            return None;
        }
        if let Some(email) = get_text_input("Enter his/her email address", InputTypes::Email) {
            friend.email = email;
        } else {
            return None;
        }
        Some(friend)
    }

    pub async fn add(&mut self, friend: NewFriend) {
        let connect = establish_connect().await;
        match connect {
            Ok(conn) => {
                let result = friend.add(&conn).await;
                match result {
                    Ok(result) => {
                        println!("New friend is add to the list! \n {:?}", result);
                    }
                    Err(err) => {
                        match err {
                            FriendError::FriendAlreadyExist => println!("Friend Already exist with this email id {}", friend.email),
                            _=> println!("Something went wrong!"),
                        }
                    },
                }
            }
            Err(err) => eprintln!("{:?}", err),
        }
        // self.friends.push(friend)
    }

    pub async fn remove(&mut self, id: i32) {
        let connect = establish_connect().await;
        match connect {
            Ok(conn) => {
                let friend = Friend::get_friend(&conn, id).await;
                match friend {
                    Ok(friend) => {
                        let table = Table::new(vec![friend.clone()]);
                        println!("{}", table);
                        let ans = Confirm::new("Do you want to remove this friend")
                            .with_default(false)
                            .with_help_message("This will be remove from friend list!")
                            .prompt();

                        match ans {
                            Ok(true) => {
                                let result = friend.remove_friend(&conn).await;
                                match result {
                                    Ok(result) => {
                                        println!("Friend is removed from te list! \n {:?}", result)
                                    }
                                    Err(_) => println!("Fail to remove friend!"),
                                }
                            }
                            Ok(false) => println!("Ok!"),
                            Err(_) => println!("Something went wrong!"),
                        }
                    }
                    Err(err) =>{
                        match err  {
                            FriendError::FriendNotFound => println!("Friend Not found!"),
                            _ => println!("Something went wrong!")
                        }
                    },
                }
            }
            Err(err) => eprintln!("{:?}", err),
        }
    }

    pub async fn show_friends(&self) {
        let connect = establish_connect().await;
        match connect {
            Ok(connect) => {
                let friends = Friend::get_friends(&connect).await;
                match friends {
                    Ok(result) => {
                        let table = Table::new(result.clone()).to_string();
                        println!("{}", table);
                    }
                    Err(_) => println!("Something went wrong, Please try again!"),
                }
            }
            Err(err) => eprintln!("{:?}",err),
        }
    }

    pub async fn get_list_of_birthday_friends()-> Result<Vec<Friend>,Error>{
        let connect = establish_connect().await?;
        let friends = sqlx::query_as!(
            Friend,
            r#"
            SELECT * FROM friend
            WHERE EXTRACT(MONTH FROM dob) = EXTRACT(MONTH FROM CURRENT_DATE)
            AND EXTRACT(DAY FROM dob) = EXTRACT(DAY FROM CURRENT_DATE)
            "#,
        ).fetch_all(&connect)
        .await;
    friends
    }
}

#[derive(Debug, Copy, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub enum BirthdayWisher {
    AddFriend,
    RemoveFriend,
    ShowFriends,
    ClearScreen,
    Exit,
}

impl BirthdayWisher {
    pub const OPTIONS: &'static [BirthdayWisher] = &[
        Self::AddFriend,
        Self::RemoveFriend,
        Self::ShowFriends,
        Self::ClearScreen,
        Self::Exit,
    ];
}

impl Display for BirthdayWisher {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let value: &str = match self {
            BirthdayWisher::AddFriend => "Add Friend",
            BirthdayWisher::RemoveFriend => "Remove Friend",
            BirthdayWisher::ShowFriends => "Show Friends",
            BirthdayWisher::ClearScreen => "Clear Screen",
            BirthdayWisher::Exit => "Exit",
        };
        write!(f, "{}", value)
    }
}
