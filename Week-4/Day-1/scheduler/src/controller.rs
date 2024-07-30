use crate::models::{Conference, Session};
use chrono::{DateTime, Utc};
use serde_json;
use std::fs::File;
use std::io::{self, Write, Read};

pub struct ConferenceController {
    pub conference: Conference,
}

impl ConferenceController {
    pub fn new(conference_name: String) -> Self {
        Self {
            conference: Conference::new(conference_name),
        }
    }

    pub fn add_session(&mut self, title: String, speaker: String, start_time: DateTime<Utc>, end_time: DateTime<Utc>, filename: &str) {
        let id = self.conference.sessions.len() as u32 + 1;
        let session = Session::new(id, title, speaker, start_time, end_time);
        self.conference.add_session(session);
        self.save_conference(filename).expect("Failed to save conference");
    }

    pub fn add_attendee_to_session(&mut self, session_id: u32, attendee: String, filename: &str) {
        if let Some(session) = self.conference.sessions.iter_mut().find(|s| s.id == session_id) {
            session.add_attendee(attendee);
            self.save_conference(filename).expect("Failed to save conference");
        } else {
            println!("Session with ID {} not found.", session_id);
        }
    }

    pub fn save_conference(&self, filename: &str) -> io::Result<()> {
        let json_data = serde_json::to_string(&self.conference)?;
        let mut file = File::create(filename)?;
        file.write_all(json_data.as_bytes())?;
        Ok(())
    }

    pub fn load_conference(&mut self, filename: &str) -> io::Result<()> {
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        self.conference = serde_json::from_str(&contents)?;
        Ok(())
    }

    pub fn list_sessions(&self) {
        for session in &self.conference.sessions {
            println!("{:?}", session);
        }
    }
}
