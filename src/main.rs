use clap::Parser;
use git_reports::ai::generate_ai_report;
use git_reports::analysis::{analyze_repo, clone_or_update};
use git_reports::config::load_config;
use serde_json;

#[derive(Parser, Debug)]
#[command(name = "git-reports")]
#[command(about = "Automated Git analytics engine for work summaries", long_about = None)]
struct Args {
    /// Ruta al archivo de configuración (config.toml)
    #[arg(short, long, default_value = "config.toml")]
    config: String,

    /// Archivo de salida JSON (por defecto imprime a stdout)
    #[arg(short, long)]
    output: Option<String>,

    /// Periodo de análisis: week, month, all
    #[arg(short, long, default_value = "week")]
    period: String,
}

fn main() {
    let args = Args::parse();

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
        config.profile.len(),
        args.period
    );

    let mut all_reports: Vec<serde_json::Value> = Vec::new();

    for profile in &config.profile {
        eprintln!("\n[perfil: {}] email: {}", profile.name, profile.email);

        let mut profile_reports: Vec<serde_json::Value> = Vec::new();

        for entry in &profile.repo {
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
                    eprintln!("  → Generando reporte IA con {} ({})...", ai_cfg.provider, ai_cfg.model);
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

    match &args.output {
        Some(path) => {
            std::fs::write(path, &json).unwrap_or_else(|e| {
                eprintln!("Error al escribir '{}': {}", path, e);
                std::process::exit(1);
            });
            eprintln!("\nReporte guardado en: {}", path);
        }
        None => println!("{}", json),
    }
}

