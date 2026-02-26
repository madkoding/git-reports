use super::{Commit, Contributor};
use serde::{Deserialize, Serialize};

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
        assert_eq!(report.total_contributors, 0);
        assert!(report.commits.is_empty());
        assert!(report.contributors.is_empty());
    }

    #[test]
    fn test_report_generated_at_format() {
        let report = Report::new("test-repo".to_string(), "week".to_string());
        // Verify it contains ISO 8601 timestamp
        assert!(report.generated_at.contains("T"));
    }

    #[test]
    fn test_report_with_commits() {
        let mut report = Report::new("test-repo".to_string(), "month".to_string());

        let commit1 = Commit {
            id: "abc123".to_string(),
            message: "First commit".to_string(),
            author: "Author 1".to_string(),
            date: "2024-01-01".to_string(),
            files_changed: 5,
            insertions: 100,
            deletions: 20,
        };

        let commit2 = Commit {
            id: "def456".to_string(),
            message: "Second commit".to_string(),
            author: "Author 1".to_string(),
            date: "2024-01-02".to_string(),
            files_changed: 3,
            insertions: 50,
            deletions: 10,
        };

        report.commits.push(commit1);
        report.commits.push(commit2);
        report.total_commits = 2;
        report.total_insertions = 150;
        report.total_deletions = 30;

        assert_eq!(report.commits.len(), 2);
        assert_eq!(report.total_commits, 2);
        assert_eq!(report.total_insertions, 150);
        assert_eq!(report.total_deletions, 30);
    }

    #[test]
    fn test_report_serialization() {
        let report = Report::new("test-repo".to_string(), "week".to_string());

        let json = serde_json::to_string(&report).unwrap();
        assert!(json.contains("test-repo"));
        assert!(json.contains("week"));
        assert!(json.contains("total_commits"));
    }

    #[test]
    fn test_report_deserialization() {
        let json = r#"{
            "repository": "test-repo",
            "period": "2weeks",
            "total_commits": 10,
            "total_contributors": 2,
            "total_insertions": 500,
            "total_deletions": 200,
            "commits": [],
            "contributors": [],
            "generated_at": "2024-01-01T00:00:00Z"
        }"#;

        let report: Report = serde_json::from_str(json).unwrap();
        assert_eq!(report.repository, "test-repo");
        assert_eq!(report.period, "2weeks");
        assert_eq!(report.total_commits, 10);
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

    #[test]
    fn test_report_add_contributor() {
        let mut report = Report::new("test-repo".to_string(), "week".to_string());

        let mut contributor =
            Contributor::new("John Doe".to_string(), "john@example.com".to_string());
        contributor.commits = 5;
        contributor.insertions = 100;
        contributor.deletions = 20;

        report.contributors.push(contributor);
        report.total_contributors = 1;

        assert_eq!(report.contributors.len(), 1);
        assert_eq!(report.total_contributors, 1);
    }
}
