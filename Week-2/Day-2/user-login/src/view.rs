
use crate::model::User;

pub struct UserView;

impl UserView {
    pub fn display_users(users: &Vec<User>) {
        for user in users {
            println!(
                "ID: {} - Name: {} - Valid: {}",
                user.id, user.name, user.valid
            );
        }
    }

    pub fn display_message(message: &str) {
        println!("{}", message);
    }
}
