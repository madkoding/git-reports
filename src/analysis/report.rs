use serde::{Deserialize, Serialize};
use super::{Commit, Contributor};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub repository: String,
    pub period: String,
    pub total_commits: u32,
    pub total_contributors: u32,
    pub total_insertions: u32,
    pub total_deletions: u32,
    pub commits: Vec<Commit>,
    pub contributors: Vec<Contributor>,
    pub generated_at: String,
}

impl Report {
    pub fn new(repository: String, period: String) -> Self {
        Self {
            repository,
            period,
            total_commits: 0,
            total_contributors: 0,
            total_insertions: 0,
            total_deletions: 0,
            commits: Vec::new(),
            contributors: Vec::new(),
            generated_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_creation() {
        let report = Report::new("test-repo".to_string(), "week".to_string());
        assert_eq!(report.repository, "test-repo");
        assert_eq!(report.period, "week");
        assert_eq!(report.total_commits, 0);
    }

    #[test]
    fn test_contributor_creation() {
        let contributor = Contributor::new("Test User".to_string(), "test@example.com".to_string());
        assert_eq!(contributor.name, "Test User");
        assert_eq!(contributor.commits, 0);
    }

    #[test]
    fn test_commit_creation() {
        let commit = Commit::new(
            "abc123".to_string(),
            "Test commit".to_string(),
            "Test Author".to_string(),
            "2024-01-01".to_string(),
        );
        assert_eq!(commit.id, "abc123");
        assert_eq!(commit.message, "Test commit");
    }
}
