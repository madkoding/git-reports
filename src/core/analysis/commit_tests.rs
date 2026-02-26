#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commit_default_values() {
        let commit = Commit::new(
            "abc123".to_string(),
            "Test commit message".to_string(),
            "Test Author".to_string(),
            "2024-01-01".to_string(),
        );

        assert_eq!(commit.id, "abc123");
        assert_eq!(commit.message, "Test commit message");
        assert_eq!(commit.author, "Test Author");
        assert_eq!(commit.date, "2024-01-01");
        assert_eq!(commit.files_changed, 0);
        assert_eq!(commit.insertions, 0);
        assert_eq!(commit.deletions, 0);
    }

    #[test]
    fn test_commit_with_stats() {
        let commit = Commit {
            id: "def456".to_string(),
            message: "Add new feature".to_string(),
            author: "John Doe".to_string(),
            date: "2024-02-15".to_string(),
            files_changed: 5,
            insertions: 100,
            deletions: 20,
        };

        assert_eq!(commit.files_changed, 5);
        assert_eq!(commit.insertions, 100);
        assert_eq!(commit.deletions, 20);
    }

    #[test]
    fn test_commit_serialization() {
        let commit = Commit::new(
            "abc123".to_string(),
            "Test commit".to_string(),
            "Author".to_string(),
            "2024-01-01".to_string(),
        );

        let json = serde_json::to_string(&commit).unwrap();
        assert!(json.contains("abc123"));
        assert!(json.contains("Test commit"));
    }

    #[test]
    fn test_commit_deserialization() {
        let json = r#"{
            "id": "abc123",
            "message": "Test commit",
            "author": "Author",
            "date": "2024-01-01",
            "files_changed": 3,
            "insertions": 50,
            "deletions": 10
        }"#;

        let commit: Commit = serde_json::from_str(json).unwrap();
        assert_eq!(commit.id, "abc123");
        assert_eq!(commit.files_changed, 3);
        assert_eq!(commit.insertions, 50);
    }
}
