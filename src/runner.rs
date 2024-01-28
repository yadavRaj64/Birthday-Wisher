use crate::{helper::utils::{clear, get_text_input}, schema::friend::{Friends, BirthdayWisher, InputTypes}};
use inquire::Select;

pub async fn send(){
    let friends = Friends::get_list_of_birthday_friends().await;
    match friends {
        Ok(friends) => {
            for i in &friends{
                i.send_birthday_email().await;
            }
        }
        Err(err) => {
            eprintln!("{:?}",err)
        },
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
