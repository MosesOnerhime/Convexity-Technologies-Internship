mod model;
mod view;
mod controller;

use std::io;

use crate::model::UserManager;
use crate::controller::UserController;
use crate::view::UserView;

struct Program {
    controller: UserController,
}

impl Program {
    fn new(controller: UserController) -> Self {
        Self { controller }
    }

    fn login(&mut self) {
        loop {
            UserView::display_message("\nInput Username: ");

            let mut user_input = String::new();
            io::stdin()
                .read_line(&mut user_input)
                .expect("Failed to read line");
            user_input = user_input.trim().to_string();

            if self.controller.valid_user(&user_input) {
                UserView::display_message("\nWelcome!");
                //UserView::display_message("\nEnd of program.");
                break;
            }

            UserView::display_message("\nWould you like to try again? (y/n): ");
            let mut redo = String::new();

            loop {
                io::stdin()
                    .read_line(&mut redo)
                    .expect("Failed to read line");
                // Trim and parse the input
                match redo.trim().to_lowercase().as_str() {
                    "y" => break,
                    "n" => {
                        //UserView::display_message("\nEnd of program.");
                        return;
                    }
                    _ => {
                        UserView::display_message("\nPlease input either (y/n): ");
                    }
                }
            }
        }
    }

    fn register(&mut self) {
        loop {
            UserView::display_message("\nInput Username: ");

            let mut user_input = String::new();
            io::stdin()
                .read_line(&mut user_input)
                .expect("Failed to read line");
            user_input = user_input.trim().to_string();

            self.controller.add_user(&user_input);
            //UserView::display_message("\nEnd of program.");
            break;
        }
    }

    fn run(&mut self) {
        UserView::display_message("𝕌 𝕊 𝔼 ℝ  𝕄 𝔸 ℕ 𝔸 𝔾 𝔼 𝕄 𝔼 ℕ 𝕋");

        loop {
            // Ask user to login or register
            UserView::display_message("\nWelcome to this User Management Application!");
            UserView::display_message("Would you like to:\n1. Login \n2. Register ");
            UserView::display_message("\nPlease select an option by inputting 1 or 2: ");

            let mut choice_input = String::new();
            io::stdin()
                .read_line(&mut choice_input)
                .expect("Failed to read line");

            // Trim and parse the input
            match choice_input.trim().parse::<i32>() {
                Ok(num) => {
                    if num == 1 {
                        println!("\nYou chose option 1.");
                        self.login();
                        break;
                    } else if num == 2 {
                        println!("\nYou chose option 2.");
                        self.register();
                        UserView::display_message("\nNow you can login.");
                        self.login();
                        break;
                    } else {
                        UserView::display_message("\nNot a valid option!");
                        UserView::display_message("\nPlease input a number between 1 & 2!\n");
                    }
                }
                Err(_) => {
                    UserView::display_message("\nNot a valid option!");
                    UserView::display_message("\nPlease input a NUMBER between 1 & 2!\n");
                }
            }
        }
    }
}

fn main() {
    let user_manager = UserManager::new();
    let controller = UserController::new(user_manager);
    let mut program = Program::new(controller);

    //program.controller.add_user("Moses Onerhime");
    //program.controller.add_user("Micheal Dowan");

    //program.controller.list_users();

    // program.controller.valid_user("Moses Onerhime");
    //program.controller.valid_user("Micheal Dowan");

    //program.controller.list_users();

    program.run();
    UserView::display_message("\nEnd of program.")
}
