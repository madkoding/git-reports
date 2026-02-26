use git_reports::ai::AiReport;
use git_reports::analysis::{Commit, Contributor, Report};
use git_reports::config::{AiConfig, RepoEntry};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

fn create_test_config(content: &str) -> (TempDir, PathBuf) {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("config.yaml");
    fs::write(&path, content).unwrap();
    (dir, path)
}

#[test]
fn test_cli_help() {
    let output = std::process::Command::new("cargo")
        .args(["run", "--", "--help"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Usage:"));
    assert!(stdout.contains("--config") || stdout.contains("-c"));
}

#[test]
fn test_cli_default_period() {
    let output = std::process::Command::new("cargo")
        .args(["run", "--", "--help"])
        .current_dir(".")
        .output()
        .expect("Failed to execute cargo run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("2weeks"));
}

#[test]
fn test_config_load_minimal() {
    let content = r#"
profiles:
  - name: "Test"
    email: "test@test.com"
    token: "token123"
    repos:
      - provider: "github"
        owner: "owner"
        name: "repo"
"#;
    let (_dir, path) = create_test_config(content);

    let config = git_reports::config::load_config(path.to_str().unwrap()).unwrap();
    assert_eq!(config.profiles.len(), 1);
    assert_eq!(config.profiles[0].repos.len(), 1);
}

#[test]
fn test_config_with_multiple_repos() {
    let content = r#"
profiles:
  - name: "Test"
    email: "test@test.com"
    token: "token123"
    repos:
      - provider: "github"
        owner: "owner1"
        name: "repo1"

      - provider: "gitlab"
        owner: "owner2"
        name: "repo2"

      - provider: "bitbucket"
        owner: "owner3"
        name: "repo3"
"#;
    let (_dir, path) = create_test_config(content);

    let config = git_reports::config::load_config(path.to_str().unwrap()).unwrap();
    assert_eq!(config.profiles[0].repos.len(), 3);
}

#[test]
fn test_report_struct() {
    let report = Report::new("test-repo".to_string(), "week".to_string());
    assert_eq!(report.repository, "test-repo");
    assert_eq!(report.period, "week");
    assert_eq!(report.total_commits, 0);
}

#[test]
fn test_commit_struct() {
    let commit = Commit::new(
        "abc123".to_string(),
        "Test commit".to_string(),
        "Author".to_string(),
        "2024-01-01".to_string(),
    );
    assert_eq!(commit.id, "abc123");
    assert_eq!(commit.message, "Test commit");
}

#[test]
fn test_contributor_struct() {
    let contributor = Contributor::new("John Doe".to_string(), "john@example.com".to_string());
    assert_eq!(contributor.name, "John Doe");
    assert_eq!(contributor.email, "john@example.com");
}

#[test]
fn test_ai_report_struct() {
    let mut hours: HashMap<String, f32> = HashMap::new();
    hours.insert("backend".to_string(), 10.0);

    let report = AiReport {
        summary: "Test summary".to_string(),
        report_markdown: "# Test\n\nContent".to_string(),
        hours_by_area: hours,
    };
    assert!(report.report_markdown.contains("Test"));
    assert!(report.summary.contains("Test"));
}

#[test]
fn test_repo_clone_url_github() {
    let repo = RepoEntry {
        provider: "github".to_string(),
        owner: "myowner".to_string(),
        name: "myrepo".to_string(),
    };

    let url = repo.clone_url("my-token");
    assert!(url.contains("github.com"));
    assert!(url.contains("myowner"));
    assert!(url.contains("myrepo"));
}

#[test]
fn test_repo_clone_url_gitlab() {
    let repo = RepoEntry {
        provider: "gitlab".to_string(),
        owner: "myowner".to_string(),
        name: "myrepo".to_string(),
    };

    let url = repo.clone_url("my-token");
    assert!(url.contains("gitlab.com"));
    assert!(url.contains("myowner"));
}

#[test]
fn test_ai_config_endpoint_ollama() {
    let ai = AiConfig {
        provider: "ollama".to_string(),
        api_key: "".to_string(),
        model: "llama3".to_string(),
        endpoint: "".to_string(),
        base_url: "http://localhost:11434".to_string(),
        headers: std::collections::HashMap::new(),
    };

    let endpoint = ai.endpoint();
    assert!(endpoint.contains("localhost:11434"));
}

#[test]
fn test_ai_config_endpoint_openai() {
    let ai = AiConfig {
        provider: "openai".to_string(),
        api_key: "sk-test".to_string(),
        model: "gpt-4".to_string(),
        endpoint: "".to_string(),
        base_url: "".to_string(),
        headers: std::collections::HashMap::new(),
    };

    let endpoint = ai.endpoint();
    assert!(endpoint.contains("openai.com"));
}
