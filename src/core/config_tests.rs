use std::fs;
use std::path::Path;
use tempfile::TempDir;

fn create_test_config(content: &str) -> (TempDir, std::path::PathBuf) {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("config.yaml");
    fs::write(&path, content).unwrap();
    (dir, path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config_basic() {
        let content = r#"
profiles:
  - name: "Test User"
    email: "test@example.com"
    token: "test-token-123"
    repos:
      - provider: "github"
        owner: "test-owner"
        name: "test-repo"
"#;
        let (_dir, path) = create_test_config(content);

        let config = crate::config::load_config(path.to_str().unwrap()).unwrap();

        assert_eq!(config.profiles.len(), 1);
        assert_eq!(config.profiles[0].name, "Test User");
        assert_eq!(config.profiles[0].email, "test@example.com");
        assert_eq!(config.profiles[0].token, "test-token-123");
        assert_eq!(config.profiles[0].repos.len(), 1);
    }

    #[test]
    fn test_load_config_multiple_profiles() {
        let content = r#"
profiles:
  - name: "User 1"
    email: "user1@example.com"
    token: "token1"
    repos:
      - provider: "github"
        owner: "owner1"
        name: "repo1"

  - name: "User 2"
    email: "user2@example.com"
    token: "token2"
    repos:
      - provider: "gitlab"
        owner: "owner2"
        name: "repo2"
"#;
        let (_dir, path) = create_test_config(content);

        let config = crate::config::load_config(path.to_str().unwrap()).unwrap();

        assert_eq!(config.profiles.len(), 2);
        assert_eq!(config.profiles[0].name, "User 1");
        assert_eq!(config.profiles[1].name, "User 2");
    }

    #[test]
    fn test_load_config_with_ai() {
        let content = r#"
profiles:
  - name: "Test User"
    email: "test@example.com"
    token: "test-token"
    ai:
      provider: "ollama"
      api_key: ""
      model: "llama3"
      base_url: "http://localhost:11434"
    repos:
      - provider: "github"
        owner: "test-owner"
        name: "test-repo"
"#;
        let (_dir, path) = create_test_config(content);

        let config = crate::config::load_config(path.to_str().unwrap()).unwrap();

        let ai = config.profiles[0].ai.as_ref().unwrap();
        assert_eq!(ai.provider, "ollama");
        assert_eq!(ai.model, "llama3");
        assert_eq!(ai.base_url, "http://localhost:11434");
    }

    #[test]
    fn test_load_config_without_ai() {
        let content = r#"
profiles:
  - name: "Test User"
    email: "test@example.com"
    token: "test-token"
    repos:
      - provider: "github"
        owner: "test-owner"
        name: "test-repo"
"#;
        let (_dir, path) = create_test_config(content);

        let config = crate::config::load_config(path.to_str().unwrap()).unwrap();

        assert!(config.profiles[0].ai.is_none());
    }

    #[test]
    fn test_load_config_file_not_found() {
        let result = crate::config::load_config("/nonexistent/path/config.yaml");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("No se pudo leer"));
    }

    #[test]
    fn test_load_config_invalid_yaml() {
        let content = r#"
profiles:
  - name: "Test User"
    email:
"#;
        let (_dir, path) = create_test_config(content);

        let result = crate::config::load_config(path.to_str().unwrap());
        assert!(result.is_err());
    }

    #[test]
    fn test_ai_config_endpoint_ollama_default() {
        let ai = crate::config::AiConfig {
            provider: "ollama".to_string(),
            api_key: "".to_string(),
            model: "llama3".to_string(),
            endpoint: "".to_string(),
            base_url: "".to_string(),
            headers: std::collections::HashMap::new(),
        };

        assert_eq!(ai.endpoint(), "http://localhost:11434/api/chat");
    }

    #[test]
    fn test_ai_config_endpoint_ollama_custom() {
        let ai = crate::config::AiConfig {
            provider: "ollama".to_string(),
            api_key: "test-key".to_string(),
            model: "llama3".to_string(),
            endpoint: "".to_string(),
            base_url: "https://ollama.com".to_string(),
            headers: std::collections::HashMap::new(),
        };

        assert_eq!(ai.endpoint(), "https://ollama.com/api/chat");
    }

    #[test]
    fn test_ai_config_endpoint_openai() {
        let ai = crate::config::AiConfig {
            provider: "openai".to_string(),
            api_key: "sk-test".to_string(),
            model: "gpt-4".to_string(),
            endpoint: "".to_string(),
            base_url: "".to_string(),
            headers: std::collections::HashMap::new(),
        };

        assert_eq!(ai.endpoint(), "https://api.openai.com/v1/chat/completions");
    }

    #[test]
    fn test_ai_config_endpoint_openai_custom() {
        let ai = crate::config::AiConfig {
            provider: "openai".to_string(),
            api_key: "sk-test".to_string(),
            model: "gpt-4".to_string(),
            endpoint: "".to_string(),
            base_url: "https://custom.openai.com".to_string(),
            headers: std::collections::HashMap::new(),
        };

        assert_eq!(
            ai.endpoint(),
            "https://custom.openai.com/v1/chat/completions"
        );
    }

    #[test]
    fn test_ai_config_endpoint_anthropic() {
        let ai = crate::config::AiConfig {
            provider: "anthropic".to_string(),
            api_key: "sk-ant-test".to_string(),
            model: "claude-3".to_string(),
            endpoint: "".to_string(),
            base_url: "".to_string(),
            headers: std::collections::HashMap::new(),
        };

        assert_eq!(ai.endpoint(), "https://api.anthropic.com/v1/messages");
    }

    #[test]
    fn test_ai_config_endpoint_custom_full() {
        let ai = crate::config::AiConfig {
            provider: "custom".to_string(),
            api_key: "test-key".to_string(),
            model: "custom-model".to_string(),
            endpoint: "https://api.miprovider.com/v1/chat".to_string(),
            base_url: "".to_string(),
            headers: std::collections::HashMap::new(),
        };

        assert_eq!(ai.endpoint(), "https://api.miprovider.com/v1/chat");
    }

    #[test]
    fn test_ai_config_headers() {
        let ai = crate::config::AiConfig {
            provider: "openai".to_string(),
            api_key: "sk-test".to_string(),
            model: "gpt-4".to_string(),
            endpoint: "".to_string(),
            base_url: "".to_string(),
            headers: std::collections::HashMap::new(),
        };

        let headers = ai.headers();
        assert!(headers.contains_key("Authorization"));
    }

    #[test]
    fn test_repo_entry_clone_url_github() {
        let repo = crate::config::RepoEntry {
            provider: "github".to_string(),
            owner: "test-owner".to_string(),
            name: "test-repo".to_string(),
        };

        let url = repo.clone_url("my-token");
        assert_eq!(url, "https://my-token@github.com/test-owner/test-repo.git");
    }

    #[test]
    fn test_repo_entry_clone_url_gitlab() {
        let repo = crate::config::RepoEntry {
            provider: "gitlab".to_string(),
            owner: "test-owner".to_string(),
            name: "test-repo".to_string(),
        };

        let url = repo.clone_url("my-token");
        assert_eq!(
            url,
            "https://gitlab-ci-token:my-token@gitlab.com/test-owner/test-repo.git"
        );
    }

    #[test]
    fn test_repo_entry_clone_url_bitbucket() {
        let repo = crate::config::RepoEntry {
            provider: "bitbucket".to_string(),
            owner: "test-owner".to_string(),
            name: "test-repo".to_string(),
        };

        let url = repo.clone_url("my-token");
        assert_eq!(
            url,
            "https://x-token-auth:my-token@bitbucket.org/test-owner/test-repo.git"
        );
    }

    #[test]
    fn test_repo_entry_cache_path() {
        let repo = crate::config::RepoEntry {
            provider: "github".to_string(),
            owner: "test-owner".to_string(),
            name: "test-repo".to_string(),
        };

        let cache = repo.cache_path("TestProfile");

        assert!(cache.to_string_lossy().contains("git-reports"));
        assert!(cache.to_string_lossy().contains("TestProfile"));
        assert!(cache.to_string_lossy().contains("github"));
        assert!(cache.to_string_lossy().contains("test-owner"));
        assert!(cache.to_string_lossy().contains("test-repo"));
    }
}
