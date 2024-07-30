mod models;
mod controller;
mod view;

use crate::controller::ConferenceController;
use crate::view::{display_sessions, display_message, prompt_input};
use chrono::Utc;

fn main() {
    display_message("Welcome to the Conference Schedule Management System!");

    let conference = prompt_input("\nInput the conference name: ");
    let mut controller = ConferenceController::new(String::from(conference));

    let filename = "conference_data.json";
    if let Err(e) = controller.load_conference(filename) {
        println!("Failed to load conference data: {}", e);
    }

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
                    controller.add_session(String::from(session), String::from(speaker), Utc::now(), Utc::now() + chrono::Duration::hours(1), filename);
                    display_message("\nSaved successfully!");
                }
                "2" => {
                    let attendee = prompt_input("\nInput the attendee's name: ");
                    let session_id = prompt_input("\nInput the session ID: ").parse::<u32>().expect("Invalid session ID");
                    controller.add_attendee_to_session(session_id, String::from(attendee), filename);
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

        loop {
            let rerun_input = prompt_input("\nWould you like to rerun? (y/n): ");
            if rerun_input == "n" {
                rerun = false;
                break;
            } else if rerun_input == "y" {
                break;
            } else if rerun_input != "y" && rerun_input != "n" {
                display_message("Please input either y or n.");
            }

            
        }

    }

    display_message("\nEnd of program.");

}
