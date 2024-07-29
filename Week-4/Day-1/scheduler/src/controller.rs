
use crate::models::{Conference, Session};
use chrono::{DateTime, Utc};

pub struct ConferenceController {
    pub conference: Conference,
}

impl ConferenceController {
    pub fn new(conference_name: String) -> Self {
        Self {
            conference: Conference::new(conference_name),
        }
    }

    pub fn add_session(&mut self, title: String, speaker: String, start_time: DateTime<Utc>, end_time: DateTime<Utc>) {
        let id = self.conference.sessions.len() as u32 + 1;
        let session = Session::new(id, title, speaker, start_time, end_time);
        self.conference.add_session(session);
    }

    pub fn add_attendee_to_session(&mut self, session_id: u32, attendee: String) {
        if let Some(session) = self.conference.sessions.iter_mut().find(|s| s.id == session_id) {
            session.add_attendee(attendee);
        } else {
            println!("Session with ID {} not found.", session_id);
        }
    }

    /*pub fn list_sessions(&self) {
        for session in &self.conference.sessions {
            println!("{:?}", session);
        }
    }*/
}
