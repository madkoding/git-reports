use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contributor {
    pub name: String,
    pub email: String,
    pub commits: u32,
    pub insertions: u32,
    pub deletions: u32,
    pub first_commit: Option<String>,
    pub last_commit: Option<String>,
}

impl Contributor {
    pub fn new(name: String, email: String) -> Self {
        Self {
            name,
            email,
            commits: 0,
            insertions: 0,
            deletions: 0,
            first_commit: None,
            last_commit: None,
        }
    }
}
