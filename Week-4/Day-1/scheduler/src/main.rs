
mod models;
mod controller;
mod view;

use crate::controller::ConferenceController;
use crate::view::{display_sessions, display_message, prompt_input};
use chrono::Utc;

fn main() {

    display_message("Welcome to this Conference Schedule Management System!");

    let conference = prompt_input("\nInput the conference name: ");

    let mut controller = ConferenceController::new(String::from(conference));

    display_message("\nSaved successfully!");

    let mut rerun: bool = true;

    while rerun {
        loop {

            display_message("\nChoose what you would like to do: \n1. Create session \n2. Add attendee to session \n3. Display sessions \n4. Exit");

            let choice: String = prompt_input("\nEnter your choice:");

            match choice.as_str() {
                "1" => {
                    let session = prompt_input("\nInput the session name: ");
                    let speaker = prompt_input("\nInput the speaker: ");
                    controller.add_session(String::from(session), String::from(speaker), Utc::now(), Utc::now() + chrono::Duration::hours(1));
                    display_message("\nSaved successfully!");
                }
                "2" => {
                    let attendee = prompt_input("\nInput the attendee's name: ");
                    controller.add_attendee_to_session(1, String::from(attendee));
                    display_message("\nSaved successfully!");
                }
                "3" => {
                    display_sessions(&controller);
                }
                "4" => {
                    display_message("Exiting...");
                    break;
                }
                _ => {
                    display_message("Invalid choice. Please enter a number between 1 - 4.");
                }
            }
        }

        let rerun_input = prompt_input("\nWould you like to rerun? (y/n): ");
        if rerun_input == "n" {
            rerun = false;
        }

        display_message("End of program.");

    }


}
