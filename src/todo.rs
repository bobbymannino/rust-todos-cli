use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Todo {
    id: u32,
    title: String,
    body: Option<String>,
    done_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
}

impl Todo {
    pub fn new(title: String, body: Option<String>) -> Self {
        Self {
            id: 0,
            title,
            body,
            done_at: None,
            created_at: Utc::now(),
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn body(&self) -> Option<&str> {
        self.body.as_deref()
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn is_done(&self) -> bool {
        self.done_at.is_some()
    }

    pub fn toggle_done(&mut self) {
        self.done_at = match self.done_at {
            Some(_) => None,
            None => Some(Utc::now()),
        };
    }
}
