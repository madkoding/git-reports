use chrono::{DateTime, Duration, Utc};
use git2::{build::RepoBuilder, Cred, FetchOptions, RemoteCallbacks, Repository};
use std::collections::HashMap;
use std::path::Path;

use super::{Commit, Contributor, Report};
use crate::config::{Profile, RepoEntry};

/// Clona el repo en la caché local o hace fetch si ya existe.
pub fn clone_or_update(entry: &RepoEntry, profile: &Profile) -> Result<Repository, String> {
    let cache_path = entry.cache_path(&profile.name);
    let url = entry.clone_url(&profile.token);

    if cache_path.exists() {
        eprintln!("  → Actualizando caché: {}", cache_path.display());
        let repo = Repository::open(&cache_path)
            .map_err(|e| format!("No se pudo abrir repo en caché: {}", e))?;

        {
            let mut remote = repo
                .find_remote("origin")
                .map_err(|e| format!("Remote 'origin' no encontrado: {}", e))?;

            let mut callbacks = RemoteCallbacks::new();
            let token = profile.token.clone();
            callbacks.credentials(move |_, _, _| Cred::userpass_plaintext(&token, ""));

            let mut fetch_opts = FetchOptions::new();
            fetch_opts.remote_callbacks(callbacks);
            fetch_opts.download_tags(git2::AutotagOption::All);

            remote
                .fetch(
                    &["refs/heads/*:refs/remotes/origin/*"],
                    Some(&mut fetch_opts),
                    None,
                )
                .map_err(|e| format!("Error en fetch: {}", e))?;
        }

        Ok(repo)
    } else {
        eprintln!("  → Clonando en: {}", cache_path.display());
        std::fs::create_dir_all(&cache_path)
            .map_err(|e| format!("No se pudo crear directorio de caché: {}", e))?;

        let mut callbacks = RemoteCallbacks::new();
        let token = profile.token.clone();
        callbacks.credentials(move |_, _, _| Cred::userpass_plaintext(&token, ""));

        let mut fetch_opts = FetchOptions::new();
        fetch_opts.remote_callbacks(callbacks);

        let mut builder = RepoBuilder::new();
        builder.fetch_options(fetch_opts);

        builder
            .clone(&url, Path::new(&cache_path))
            .map_err(|e| format!("Error al clonar '{}': {}", url, e))
    }
}

/// Analiza los commits del repo filtrando por periodo y email del autor.
pub fn analyze_repo(
    repo: &Repository,
    period: &str,
    email: &str,
    repo_name: &str,
) -> Result<Report, String> {
    let mut report = Report::new(repo_name.to_string(), period.to_string());

    let cutoff: Option<DateTime<Utc>> = match period {
        "2weeks" => Some(Utc::now() - Duration::days(14)),
        "week" => Some(Utc::now() - Duration::days(7)),
        "month" => Some(Utc::now() - Duration::days(30)),
        _ => None,
    };

    // Determinar HEAD; si el repo está vacío se devuelve un reporte vacío
    let head = match repo.head() {
        Ok(h) => h,
        Err(_) => return Ok(report),
    };
    let head_commit = head.peel_to_commit().map_err(|e| e.to_string())?;

    let mut revwalk = repo.revwalk().map_err(|e| e.to_string())?;
    revwalk.push(head_commit.id()).map_err(|e| e.to_string())?;
    revwalk
        .set_sorting(git2::Sort::TIME)
        .map_err(|e| e.to_string())?;

    let mut contributors: HashMap<String, Contributor> = HashMap::new();

    for oid in revwalk {
        let oid = oid.map_err(|e| e.to_string())?;
        let commit = repo.find_commit(oid).map_err(|e| e.to_string())?;

        let author = commit.author();
        let commit_email = author.email().unwrap_or("").to_string();
        let commit_name = author.name().unwrap_or("Unknown").to_string();

        // Filtrar por email del perfil
        if commit_email != email {
            continue;
        }

        // Filtrar por periodo
        let commit_time = commit.time();
        let commit_dt: DateTime<Utc> =
            DateTime::from_timestamp(commit_time.seconds(), 0).unwrap_or(Utc::now());

        if let Some(cutoff_dt) = cutoff {
            if commit_dt < cutoff_dt {
                break; // revwalk va de más nuevo a más viejo; si ya pasó el corte, salimos
            }
        }

        // Calcular diff stats respecto al primer padre
        let (files_changed, insertions, deletions) = diff_stats(repo, &commit);

        let commit_id = format!("{:.8}", oid);
        let message = commit
            .message()
            .unwrap_or("")
            .lines()
            .next()
            .unwrap_or("")
            .to_string();
        let date = commit_dt.format("%Y-%m-%dT%H:%M:%SZ").to_string();

        let mut c = Commit::new(commit_id, message, commit_name.clone(), date);
        c.files_changed = files_changed;
        c.insertions = insertions;
        c.deletions = deletions;

        report.total_commits += 1;
        report.total_insertions += insertions;
        report.total_deletions += deletions;
        report.commits.push(c);

        // Acumular contribuidor
        let contrib = contributors
            .entry(commit_email.clone())
            .or_insert_with(|| Contributor::new(commit_name.clone(), commit_email.clone()));
        contrib.commits += 1;
        contrib.insertions += insertions;
        contrib.deletions += deletions;

        let date_str = commit_dt.format("%Y-%m-%dT%H:%M:%SZ").to_string();
        if contrib.first_commit.is_none() {
            contrib.last_commit = Some(date_str.clone());
        }
        contrib.first_commit = Some(date_str);
    }

    report.contributors = contributors.into_values().collect();
    report.total_contributors = report.contributors.len() as u32;

    Ok(report)
}

fn diff_stats(repo: &Repository, commit: &git2::Commit) -> (u32, u32, u32) {
    let tree = match commit.tree() {
        Ok(t) => t,
        Err(_) => return (0, 0, 0),
    };

    let parent_tree = commit.parent(0).ok().and_then(|p| p.tree().ok());

    let diff = match repo.diff_tree_to_tree(parent_tree.as_ref(), Some(&tree), None) {
        Ok(d) => d,
        Err(_) => return (0, 0, 0),
    };

    let stats = match diff.stats() {
        Ok(s) => s,
        Err(_) => return (0, 0, 0),
    };

    (
        stats.files_changed() as u32,
        stats.insertions() as u32,
        stats.deletions() as u32,
    )
}
