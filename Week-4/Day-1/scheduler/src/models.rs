use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub id: u32,
    pub title: String,
    pub speaker: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub attendees: Vec<String>,
}

impl Session {
    pub fn new(id: u32, title: String, speaker: String, start_time: DateTime<Utc>, end_time: DateTime<Utc>) -> Self {
        Self {
            id,
            title,
            speaker,
            start_time,
            end_time,
            attendees: Vec::new(),
        }
    }

    pub fn add_attendee(&mut self, attendee: String) {
        self.attendees.push(attendee);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Conference {
    pub name: String,
    pub sessions: Vec<Session>,
}

impl Conference {
    pub fn new(name: String) -> Self {
        Self {
            name,
            sessions: Vec::new(),
        }
    }

    pub fn add_session(&mut self, session: Session) {
        self.sessions.push(session);
    }

    /*pub fn get_sessions(&self) -> &Vec<Session> {
        &self.sessions
    }*/
}
