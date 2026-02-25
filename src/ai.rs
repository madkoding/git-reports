use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::analysis::Report;
use crate::config::AiConfig;

/// Resultado del análisis de IA para un periodo y repositorio.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiReport {
    /// Resumen ejecutivo breve (2-4 líneas).
    pub summary: String,
    /// Reporte listo para enviar en formato Markdown.
    pub report_markdown: String,
    /// Estimación de horas por área de trabajo.
    pub hours_by_area: std::collections::HashMap<String, f32>,
}

/// Genera un reporte de IA a partir de los commits del `Report`.
/// Devuelve `None` si no hay commits para analizar.
pub fn generate_ai_report(
    report: &Report,
    ai: &AiConfig,
    profile_name: &str,
) -> Result<Option<AiReport>, String> {
    if report.commits.is_empty() {
        return Ok(None);
    }

    let commits_text: String = report
        .commits
        .iter()
        .map(|c| {
            format!(
                "- [{}] {} (+{} -{} líneas, {} archivos)",
                c.date, c.message, c.insertions, c.deletions, c.files_changed
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let prompt = format!(
        r#"Eres un asistente técnico que analiza actividad de desarrollo de software.

Perfil: {profile_name}
Repositorio: {repo}
Periodo: {period}
Total commits: {total_commits}
Inserciones: {insertions} líneas | Deleciones: {deletions} líneas

Commits:
{commits}

Basándote en los mensajes de commit y las estadísticas, genera un JSON con EXACTAMENTE esta estructura:
{{
  "summary": "<resumen ejecutivo del trabajo realizado, 2-4 oraciones en primera persona>",
  "report_markdown": "<reporte profesional completo en Markdown, listo para enviar a un cliente o supervisor>",
  "hours_by_area": {{
    "<área1>": <horas estimadas como número>,
    "<área2>": <horas estimadas como número>
  }}
}}

Para hours_by_area, clasifica el trabajo en áreas según los commits (backend, frontend, devops, testing, documentación, etc.) y estima horas razonables. Responde SOLO con el JSON, sin texto adicional."#,
        profile_name = profile_name,
        repo = report.repository,
        period = report.period,
        total_commits = report.total_commits,
        insertions = report.total_insertions,
        deletions = report.total_deletions,
        commits = commits_text,
    );

    let response_text = match ai.provider.as_str() {
        "openai" => call_openai(&prompt, ai)?,
        "anthropic" => call_anthropic(&prompt, ai)?,
        "ollama" => call_ollama(&prompt, ai)?,
        other => return Err(format!("Provider de IA no soportado: '{}'", other)),
    };

    parse_ai_response(&response_text)
        .map(Some)
        .map_err(|e| format!("Error al parsear respuesta de IA: {}\nRespuesta: {}", e, response_text))
}

fn call_openai(prompt: &str, ai: &AiConfig) -> Result<String, String> {
    let client = reqwest::blocking::Client::new();
    let body = json!({
        "model": ai.model,
        "messages": [{"role": "user", "content": prompt}],
        "response_format": {"type": "json_object"}
    });

    let resp = client
        .post(ai.endpoint())
        .header("Authorization", format!("Bearer {}", ai.api_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .map_err(|e| format!("Error en request a OpenAI: {}", e))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().unwrap_or_default();
        return Err(format!("OpenAI error {}: {}", status, text));
    }

    let json: Value = resp.json().map_err(|e| format!("Error leyendo respuesta OpenAI: {}", e))?;
    json["choices"][0]["message"]["content"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Respuesta inesperada de OpenAI: {}", json))
}

fn call_anthropic(prompt: &str, ai: &AiConfig) -> Result<String, String> {
    let client = reqwest::blocking::Client::new();
    let body = json!({
        "model": ai.model,
        "max_tokens": 2048,
        "messages": [{"role": "user", "content": prompt}]
    });

    let resp = client
        .post(ai.endpoint())
        .header("x-api-key", &ai.api_key)
        .header("anthropic-version", "2023-06-01")
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .map_err(|e| format!("Error en request a Anthropic: {}", e))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().unwrap_or_default();
        return Err(format!("Anthropic error {}: {}", status, text));
    }

    let json: Value = resp.json().map_err(|e| format!("Error leyendo respuesta Anthropic: {}", e))?;
    json["content"][0]["text"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Respuesta inesperada de Anthropic: {}", json))
}

fn call_ollama(prompt: &str, ai: &AiConfig) -> Result<String, String> {
    let client = reqwest::blocking::Client::new();
    let body = json!({
        "model": ai.model,
        "messages": [{"role": "user", "content": prompt}],
        "stream": false,
        "format": "json"
    });

    let resp = client
        .post(ai.endpoint())
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .map_err(|e| format!("Error en request a Ollama: {}", e))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().unwrap_or_default();
        return Err(format!("Ollama error {}: {}", status, text));
    }

    let json: Value = resp.json().map_err(|e| format!("Error leyendo respuesta Ollama: {}", e))?;
    json["message"]["content"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Respuesta inesperada de Ollama: {}", json))
}

fn parse_ai_response(text: &str) -> Result<AiReport, String> {
    // Intentar parsear directamente
    if let Ok(report) = serde_json::from_str::<AiReport>(text) {
        return Ok(report);
    }

    // Si la respuesta viene envuelta en un bloque de código ```json ... ```
    let cleaned = text
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    serde_json::from_str::<AiReport>(cleaned)
        .map_err(|e| format!("JSON inválido: {}", e))
}
