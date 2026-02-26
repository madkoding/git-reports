use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(alias = "profile", alias = "profiles")]
    pub profiles: Vec<Profile>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AiConfig {
    /// Provider predefinido: "openai" | "anthropic" | "ollama"
    /// Si se define `endpoint`, este campo es opcional
    #[serde(default)]
    pub provider: String,
    /// API key
    #[serde(default)]
    pub api_key: String,
    /// Modelo a usar
    pub model: String,
    /// Endpoint completo de la API. Si se define, ignora `provider` y `base_url`
    /// Ejemplo: "https://api.openai.com/v1/chat/completions"
    #[serde(default)]
    pub endpoint: String,
    /// URL base. Se combina con el provider si endpoint no está definido
    /// Ejemplo: "https://api.openai.com" o "http://localhost:11434"
    #[serde(default)]
    pub base_url: String,
    /// Headers adicionales para la petición
    #[serde(default)]
    pub headers: std::collections::HashMap<String, String>,
}

impl AiConfig {
    /// Retorna el endpoint completo para la API
    pub fn endpoint(&self) -> String {
        // Si sedefine endpoint explícito, usarlo
        if !self.endpoint.is_empty() {
            return self.endpoint.clone();
        }

        // Si hay base_url, usarla
        let base = if !self.base_url.is_empty() {
            self.base_url.trim_end_matches('/').to_string()
        } else {
            // Usar defaults según provider
            match self.provider.as_str() {
                "openai" => "https://api.openai.com".to_string(),
                "anthropic" => "https://api.anthropic.com".to_string(),
                "ollama" => "http://localhost:11434".to_string(),
                _ => "http://localhost:11434".to_string(),
            }
        };

        // Añadir path según provider
        match self.provider.as_str() {
            "openai" => format!("{}/v1/chat/completions", base),
            "anthropic" => format!("{}/v1/messages", base),
            _ => format!("{}/api/chat", base),
        }
    }

    /// Retorna los headers para la petición, incluyendo Authorization si hay api_key
    pub fn headers(&self) -> std::collections::HashMap<String, String> {
        let mut headers = self.headers.clone();

        if !self.api_key.is_empty() {
            let auth_value = match self.provider.as_str() {
                "anthropic" => format!("Bearer {}", self.api_key),
                _ => self.api_key.clone(),
            };
            headers
                .entry("Authorization".to_string())
                .or_insert(auth_value);
        }

        headers
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Profile {
    pub name: String,
    pub email: String,
    pub token: String,
    /// Configuración de IA para este perfil (opcional)
    pub ai: Option<AiConfig>,
    #[serde(alias = "repo", default)]
    pub repos: Vec<RepoEntry>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RepoEntry {
    pub provider: String, // github | gitlab | bitbucket
    pub owner: String,
    pub name: String,
}

impl RepoEntry {
    /// Construye la URL de clone con el token embebido según el provider.
    pub fn clone_url(&self, token: &str) -> String {
        match self.provider.as_str() {
            "github" => format!(
                "https://{}@github.com/{}/{}.git",
                token, self.owner, self.name
            ),
            "gitlab" => format!(
                "https://gitlab-ci-token:{}@gitlab.com/{}/{}.git",
                token, self.owner, self.name
            ),
            "bitbucket" => format!(
                "https://x-token-auth:{}@bitbucket.org/{}/{}.git",
                token, self.owner, self.name
            ),
            other => format!(
                "https://{}@{}/{}/{}.git",
                token, other, self.owner, self.name
            ),
        }
    }

    /// Devuelve la ruta de caché local para este repo.
    /// ~/.cache/git-reports/{profile_name}/{provider}/{owner}/{name}
    pub fn cache_path(&self, profile_name: &str) -> std::path::PathBuf {
        dirs_next::cache_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("/tmp"))
            .join("git-reports")
            .join(profile_name)
            .join(&self.provider)
            .join(&self.owner)
            .join(&self.name)
    }
}

/// Carga y deserializa el archivo de configuración (soporta YAML y JSON).
pub fn load_config(path: &str) -> Result<Config, String> {
    let content = fs::read_to_string(Path::new(path))
        .map_err(|e| format!("No se pudo leer '{}': {}", path, e))?;

    // Intentar primero como YAML
    if let Ok(config) = serde_yaml::from_str::<Config>(&content) {
        return Ok(config);
    }

    // Si falla, intentar como JSON
    match serde_json::from_str::<Config>(&content) {
        Ok(config) => Ok(config),
        Err(e) => Err(format!(
            "Error al parsear '{}': {} (probablemente no es YAML válido)",
            path, e
        )),
    }
}
