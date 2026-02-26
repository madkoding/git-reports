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

    let response_text = call_ai(&prompt, ai)?;
    parse_ai_response(&response_text).map(Some).map_err(|e| {
        format!(
            "Error al parsear respuesta de IA: {}\nRespuesta: {}",
            e, response_text
        )
    })
}

/// Realiza la llamada a la API de IA de forma genérica basada en la configuración
fn call_ai(prompt: &str, ai: &AiConfig) -> Result<String, String> {
    let client = reqwest::blocking::Client::new();
    let endpoint = ai.endpoint();
    let headers = ai.headers();

    // Construir body según el provider o formato custom
    let body = build_request_body(prompt, ai);

    // Construir request
    let mut req = client.post(&endpoint);

    // Agregar headers
    for (key, value) in &headers {
        req = req.header(key.as_str(), value.as_str());
    }

    // Content-Type por defecto
    if !headers.contains_key("Content-Type") {
        req = req.header("Content-Type", "application/json");
    }

    req = req.json(&body);

    let resp = req
        .send()
        .map_err(|e| format!("Error en request a IA: {}", e))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().unwrap_or_default();
        return Err(format!("IA error {}: {}", status, text));
    }

    // Parsear respuesta
    let json: Value = resp
        .json()
        .map_err(|e| format!("Error leyendo respuesta: {}", e))?;

    extract_response_content(&json, ai)
}

/// Construye el body de la petición según el provider
fn build_request_body(prompt: &str, ai: &AiConfig) -> Value {
    // Si hay un formato custom definido en headers, usar prompt directo
    // Esto permite compatibilidad con providers que aceptan texto plano

    match ai.provider.as_str() {
        "openai" | "custom" if !ai.provider.is_empty() => {
            json!({
                "model": ai.model,
                "messages": [{"role": "user", "content": prompt}],
                "response_format": {"type": "json_object"}
            })
        }
        "anthropic" => json!({
            "model": ai.model,
            "max_tokens": 4096,
            "messages": [{"role": "user", "content": prompt}]
        }),
        // Ollama y default
        _ => json!({
            "model": ai.model,
            "prompt": prompt,
            "stream": false,
            "format": "json"
        }),
    }
}

/// Extrae el contenido de la respuesta según el provider
fn extract_response_content(json: &Value, ai: &AiConfig) -> Result<String, String> {
    match ai.provider.as_str() {
        "openai" => json["choices"][0]["message"]["content"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| format!("Respuesta inesperada de OpenAI: {}", json)),
        "anthropic" => json["content"][0]["text"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| format!("Respuesta inesperada de Anthropic: {}", json)),
        // Ollama y default
        _ => json["response"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| format!("Respuesta inesperada: {}", json)),
    }
}

/// Extrae el JSON de la respuesta de la IA
fn parse_ai_response(text: &str) -> Result<AiReport, String> {
    // Limpiar caracteres de control
    let cleaned_input: String = text.chars().filter(|c| !c.is_ascii_control()).collect();

    // Intentar parsear directamente
    if let Ok(report) = serde_json::from_str::<AiReport>(&cleaned_input) {
        return Ok(report);
    }

    // Buscar el bloque JSON: buscar desde ```json hasta ```
    if let Some(start) = cleaned_input.find("```json") {
        let rest = &cleaned_input[start + 7..];
        if let Some(end) = rest.find("```") {
            let json_str = &rest[..end];
            if let Ok(report) = serde_json::from_str::<AiReport>(json_str) {
                return Ok(report);
            }
        }
    }

    Err("No se pudo parsear la respuesta como JSON".to_string())
}
