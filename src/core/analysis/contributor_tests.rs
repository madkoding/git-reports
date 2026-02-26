#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contributor_default_values() {
        let contributor = Contributor::new("John Doe".to_string(), "john@example.com".to_string());

        assert_eq!(contributor.name, "John Doe");
        assert_eq!(contributor.email, "john@example.com");
        assert_eq!(contributor.commits, 0);
        assert_eq!(contributor.insertions, 0);
        assert_eq!(contributor.deletions, 0);
        assert!(contributor.first_commit.is_none());
        assert!(contributor.last_commit.is_none());
    }

    #[test]
    fn test_contributor_with_all_fields() {
        let contributor = Contributor {
            name: "Jane Doe".to_string(),
            email: "jane@example.com".to_string(),
            commits: 25,
            insertions: 500,
            deletions: 200,
            first_commit: Some("2024-01-01".to_string()),
            last_commit: Some("2024-02-15".to_string()),
        };

        assert_eq!(contributor.commits, 25);
        assert_eq!(contributor.insertions, 500);
        assert_eq!(contributor.deletions, 200);
        assert_eq!(contributor.first_commit.unwrap(), "2024-01-01");
        assert_eq!(contributor.last_commit.unwrap(), "2024-02-15");
    }

    #[test]
    fn test_contributor_serialization() {
        let contributor = Contributor::new("John Doe".to_string(), "john@example.com".to_string());

        let json = serde_json::to_string(&contributor).unwrap();
        assert!(json.contains("John Doe"));
        assert!(json.contains("john@example.com"));
    }

    #[test]
    fn test_contributor_deserialization() {
        let json = r#"{
            "name": "John Doe",
            "email": "john@example.com",
            "commits": 10,
            "insertions": 200,
            "deletions": 50,
            "first_commit": "2024-01-01",
            "last_commit": "2024-02-01"
        }"#;

        let contributor: Contributor = serde_json::from_str(json).unwrap();
        assert_eq!(contributor.name, "John Doe");
        assert_eq!(contributor.commits, 10);
        assert_eq!(contributor.insertions, 200);
    }
}
