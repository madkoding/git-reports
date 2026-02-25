use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub profile: Vec<Profile>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Profile {
    pub name: String,
    pub email: String,
    pub token: String,
    #[serde(default)]
    pub repo: Vec<RepoEntry>,
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
                "https://oauth2:{}@gitlab.com/{}/{}.git",
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
        let base = dirs_next::cache_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("/tmp"))
            .join("git-reports")
            .join(profile_name)
            .join(&self.provider)
            .join(&self.owner)
            .join(&self.name);
        base
    }
}

/// Carga y deserializa el archivo config.toml desde la ruta indicada.
pub fn load_config(path: &str) -> Result<Config, String> {
    let content = fs::read_to_string(Path::new(path))
        .map_err(|e| format!("No se pudo leer '{}': {}", path, e))?;
    toml::from_str::<Config>(&content)
        .map_err(|e| format!("Error al parsear '{}': {}", path, e))
}
