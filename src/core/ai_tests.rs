#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_report_creation() {
        let report = AiReport {
            report_markdown: "# Test Report\n\nThis is a test.".to_string(),
        };
        
        assert!(report.report_markdown.contains("# Test Report"));
    }

    #[test]
    fn test_ai_report_serialization() {
        let report = AiReport {
            report_markdown: "# Test\n\nContent".to_string(),
        };
        
        let json = serde_json::to_string(&report).unwrap();
        assert!(json.contains("report_markdown"));
        assert!(json.contains("Test"));
    }

    #[test]
    fn test_ai_report_deserialization() {
        let json = r#"{
            "report_markdown": "# Hello\n\nThis is a test report."
        }"#;
        
        let report: AiReport = serde_json::from_str(json).unwrap();
        assert!(report.report_markdown.contains("Hello"));
    }

    #[test]
    fn test_generate_ai_report_empty_commits() {
        let report = Report::new("test-repo".to_string(), "week".to_string());
        
        let ai = AiConfig {
            provider: "ollama".to_string(),
            api_key: "".to_string(),
            model: "llama3".to_string(),
            base_url: None,
        };
        
        let result = generate_ai_report(&report, &ai, "Test User");
        
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_ai_report_with_commits() {
        let mut report = Report::new("test-repo".to_string(), "week".to_string());
        
        let commit = Commit {
            id: "abc123".to_string(),
            message: "Add new feature".to_string(),
            author: "John Doe".to_string(),
            date: "2024-01-01".to_string(),
            files_changed: 5,
            insertions: 100,
            deletions: 20,
        };
        
        report.commits.push(commit);
        report.total_commits = 1;
        report.total_insertions = 100;
        report.total_deletions = 20;
        
        assert!(!report.commits.is_empty());
        assert_eq!(report.total_commits, 1);
    }

    #[test]
    fn test_ai_report_markdown_formatting() {
        let markdown = r#"# Titulo
## Seccion 1
Contenido de prueba
- Item 1
- Item 2
## Seccion 2
Mas contenido"#;
        
        let report = AiReport {
            report_markdown: markdown.to_string(),
        };
        
        assert!(report.report_markdown.contains("# Titulo"));
        assert!(report.report_markdown.contains("- Item 1"));
    }
}
