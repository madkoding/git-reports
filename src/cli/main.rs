use chrono::{Duration, Utc};
use clap::Parser;
use git_reports::ai::generate_ai_report;
use git_reports::analysis::{analyze_repo, clone_or_update};
use git_reports::config::load_config;

#[derive(Parser, Debug)]
#[command(name = "git-reports")]
#[command(about = "Automated Git analytics engine for work summaries", long_about = None)]
struct Args {
    /// Ruta al archivo de configuración (config.yaml)
    #[arg(short, long, default_value = "config.yaml")]
    config: String,

    /// Directorio de salida para los reportes (default: "reports")
    #[arg(short, long, default_value = "reports")]
    output_dir: String,

    /// Nombre base del reporte (sin extensión). Se agregará la fecha automáticamente
    #[arg(short, long, default_value = "report")]
    name: String,

    /// Periodo de análisis: 2weeks, week, month, all
    #[arg(short, long, default_value = "2weeks")]
    period: String,
}

fn main() {
    let args = Args::parse();

    // Obtener el directorio donde está el ejecutable
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| std::path::PathBuf::from("."));

    // Usar el directorio del ejecutable como base para reports
    let output_dir = if args.output_dir.starts_with('/') || args.output_dir.contains(':') {
        // Es una ruta absoluta
        std::path::PathBuf::from(&args.output_dir)
    } else {
        // Es ruta relativa - usar directorio del ejecutable
        exe_dir.join(&args.output_dir)
    };

    // Calcular fechas para el nombre del archivo
    let now = Utc::now();
    let days = match args.period.as_str() {
        "week" => 7,
        "2weeks" => 14,
        "month" => 30,
        _ => 14,
    };
    let start_date = now - Duration::days(days);
    let date_range = format!(
        "{}_{}",
        start_date.format("%Y-%m-%d"),
        now.format("%Y-%m-%d")
    );

    // Crear directorio de salida si no existe
    std::fs::create_dir_all(&output_dir).unwrap_or_else(|e| {
        eprintln!(
            "Error al crear directorio '{}': {}",
            output_dir.display(),
            e
        );
        std::process::exit(1);
    });

    // Rutas de salida
    let json_path = format!("{}/{}_{}.json", output_dir.display(), args.name, date_range);
    let md_path = format!("{}/{}_{}.md", output_dir.display(), args.name, date_range);

    // Cargar configuración
    let config = match load_config(&args.config) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    eprintln!(
        "Git Reports — {} perfil(es) encontrado(s), periodo: {}",
        config.profiles.len(),
        args.period
    );

    let mut all_reports: Vec<serde_json::Value> = Vec::new();

    for profile in &config.profiles {
        eprintln!("\n[perfil: {}] email: {}", profile.name, profile.email);

        let mut profile_reports: Vec<serde_json::Value> = Vec::new();

        for entry in &profile.repos {
            let label = format!("{}/{}/{}", entry.provider, entry.owner, entry.name);
            eprintln!("  Procesando {}", label);

            // Clonar o actualizar el repo en caché
            let repo = match clone_or_update(entry, profile) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("  ✗ Error: {}", e);
                    continue;
                }
            };

            // Analizar commits filtrados por email y periodo
            let report = match analyze_repo(&repo, &args.period, &profile.email, &label) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("  ✗ Error al analizar: {}", e);
                    continue;
                }
            };

            eprintln!(
                "  ✓ {} commit(s) encontrado(s) para {}",
                report.total_commits, profile.email
            );

            // Análisis de IA (si está configurado en el perfil)
            let ai_report = match &profile.ai {
                Some(ai_cfg) => {
                    eprintln!(
                        "  → Generando reporte IA con {} ({})...",
                        ai_cfg.provider, ai_cfg.model
                    );
                    match generate_ai_report(&report, ai_cfg, &profile.name) {
                        Ok(r) => {
                            eprintln!("  ✓ Reporte IA generado");
                            r.map(|r| serde_json::to_value(r).unwrap())
                        }
                        Err(e) => {
                            eprintln!("  ✗ Error IA: {}", e);
                            None
                        }
                    }
                }
                None => None,
            };

            let mut repo_value = serde_json::to_value(&report).unwrap();
            if let Some(ai) = ai_report {
                repo_value["ai_report"] = ai;
            }
            profile_reports.push(repo_value);
        }

        all_reports.push(serde_json::json!({
            "profile": profile.name,
            "email": profile.email,
            "repos": profile_reports,
        }));
    }

    // Serializar resultado final
    let json = serde_json::to_string_pretty(&all_reports).unwrap_or_else(|e| {
        eprintln!("Error al serializar JSON: {}", e);
        std::process::exit(1);
    });

    // Guardar JSON
    std::fs::write(&json_path, &json).unwrap_or_else(|e| {
        eprintln!("Error al escribir '{}': {}", json_path, e);
        std::process::exit(1);
    });
    eprintln!("\nReporte JSON guardado en: {}", json_path);

    // Generar y guardar Markdown
    let mut markdown = String::new();
    for profile_data in &all_reports {
        if let Some(repos) = profile_data.get("repos").and_then(|r| r.as_array()) {
            for repo_data in repos {
                let repo_name = repo_data
                    .get("repository")
                    .and_then(|r| r.as_str())
                    .unwrap_or("unknown");

                let repo_total_commits = repo_data
                    .get("total_commits")
                    .and_then(|r| r.as_u64())
                    .unwrap_or(0);

                // Verificar si hay ai_report
                if let Some(ai_report) = repo_data.get("ai_report") {
                    if let Some(report_md) =
                        ai_report.get("report_markdown").and_then(|r| r.as_str())
                    {
                        // Limpiar el markdown: quitar bloques de código wrapper
                        let mut final_md = report_md.replace("\\n", "\n");

                        // Quitar ```markdown y ``` del inicio y final
                        final_md = final_md
                            .trim_start_matches("```markdown\n")
                            .trim_start_matches("```markdown")
                            .trim_start_matches("```")
                            .trim_start_matches("\n```")
                            .to_string();
                        final_md = final_md
                            .trim_end_matches("```")
                            .trim_end_matches("\n```")
                            .trim()
                            .to_string();

                        // Limpiar caracteres de control
                        final_md = final_md
                            .chars()
                            .filter(|c| !c.is_ascii_control() || *c == '\n')
                            .collect();

                        markdown.push_str(&format!("# {}\n\n", repo_name));
                        markdown.push_str(&final_md);
                        markdown.push_str("\n\n---\n\n");
                    }
                } else if repo_total_commits > 0 {
                    // Si no hay IA pero hay commits, generar un resumen básico
                    let total_commits = repo_total_commits;
                    let total_insertions = repo_data
                        .get("total_insertions")
                        .and_then(|r| r.as_u64())
                        .unwrap_or(0);
                    let total_deletions = repo_data
                        .get("total_deletions")
                        .and_then(|r| r.as_u64())
                        .unwrap_or(0);

                    let mut commits_text = String::new();
                    if let Some(commits) = repo_data.get("commits").and_then(|c| c.as_array()) {
                        for commit in commits.iter().take(10) {
                            let msg = commit.get("message").and_then(|m| m.as_str()).unwrap_or("");
                            let date = commit.get("date").and_then(|d| d.as_str()).unwrap_or("");
                            commits_text.push_str(&format!("- {}: {}\n", date, msg));
                        }
                    }

                    markdown.push_str(&format!("# {}\n\n", repo_name));
                    markdown.push_str("## Resumen\n\n");
                    markdown.push_str(&format!("- Total commits: {}\n", total_commits));
                    markdown.push_str(&format!("- Líneas añadidas: {}\n", total_insertions));
                    markdown.push_str(&format!("- Líneas eliminadas: {}\n\n", total_deletions));
                    markdown.push_str("## Commits recientes\n\n");
                    markdown.push_str(&commits_text);
                    markdown.push_str("\n\n---\n\n");
                }
            }
        }
    }

    if markdown.is_empty() {
        eprintln!("No se generó contenido para el reporte MD");
    } else {
        std::fs::write(&md_path, &markdown).unwrap_or_else(|e| {
            eprintln!("Error al escribir '{}': {}", md_path, e);
            std::process::exit(1);
        });
        eprintln!("Reporte MD guardado en: {}", md_path);
    }
}
