
use crate::controller::ConferenceController;


pub fn display_sessions(controller: &ConferenceController) {
    println!("Sessions for conference '{}':", controller.conference.name);
    for session in &controller.conference.sessions {
        println!("ID: {}, Title: {}, Speaker: {}, Start: {}, End: {}", 
            session.id, session.title, session.speaker, session.start_time, session.end_time);
    }
}

pub fn display_message(message: &str) {
    println!("{}", message);
}

pub fn prompt_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_lowercase().to_string()
}
