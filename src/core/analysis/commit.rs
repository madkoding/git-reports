use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    pub id: String,
    pub message: String,
    pub author: String,
    pub date: String,
    pub files_changed: u32,
    pub insertions: u32,
    pub deletions: u32,
}

impl Commit {
    pub fn new(id: String, message: String, author: String, date: String) -> Self {
        Self {
            id,
            message,
            author,
            date,
            files_changed: 0,
            insertions: 0,
            deletions: 0,
        }
    }
}
