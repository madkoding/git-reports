# Git Reports

[![CI](https://img.shields.io/github/actions/workflow/status/madkoding/git-reports/ci.yml?label=CI)](https://github.com/madkoding/git-reports/actions)
[![Release](https://img.shields.io/github/v/release/madkoding/git-reports?label=Version)](https://github.com/madkoding/git-reports/releases)
[![Rust](https://img.shields.io/badge/Rust-1.70+-DC2626?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License](https://img.shields.io/github/license/madkoding/git-reports?label=License)](https://github.com/madkoding/git-reports/blob/main/LICENSE)

Generador de reportes automatizados para repositorios Git con análisis de IA.

## 📋 Descripción

Git Reports analiza tus commits de Git y genera reportes profesionales en formato Markdown estilo Jira, listos para presentar como parte de tu trabajo.

## 🚀 Instalación

### Opción 1: Descargar ejecutable

Descarga el ejecutable para tu sistema operativo desde [Releases](https://github.com/madkoding/git-reports/releases):

```bash
# Linux
wget https://github.com/madkoding/git-reports/releases/latest/download/git-reports-linux-x64
chmod +x git-reports-linux-x64
./git-reports-linux-x64

# macOS
curl -L https://github.com/madkoding/git-reports/releases/latest/download/git-reports-macos-x64 -o git-reports
chmod +x git-reports
./git-reports

# Windows
# Descargar desde releases y ejecutar en PowerShell
.\git-reports-windows-x64.exe
```

### Opción 2: Compilar desde código

```bash
git clone https://github.com/madkoding/git-reports.git
cd git-reports
cargo build --release
./target/release/git-reports
```

## ⚙️ Configuración

Crea un archivo `config.yaml` en el mismo directorio que el ejecutable:

```yaml
# config.yaml
profiles:
  - name: "Tu Nombre"
    email: "tu@email.com"
    token: "tu-gitlab-token"
    
    # Configuración de IA (opcional)
    ai:
      provider: "ollama"      # openai, anthropic, ollama
      api_key: ""            # no requerido para ollama local
      model: "llama3"
      base_url: "http://localhost:11434"
    
    repos:
      - provider: "gitlab"
        owner: "tu-organizacion"
        name: "tu-repositorio"
```

### Parámetros de configuración

| Campo | Descripción | Obligatorio |
|-------|-------------|-------------|
| `profiles[].name` | Nombre del perfil | Sí |
| `profiles[].email` | Email del desarrollador | Sí |
| `profiles[].token` | Token de acceso (GitLab PAT, GitHub PAT, etc.) | Sí |
| `profiles[].ai` | Configuración de IA (opcional) | No |
| `profiles[].ai.provider` | Provider: `openai`, `anthropic`, `ollama` | Sí* |
| `profiles[].ai.api_key` | API Key del provider | No |
| `profiles[].ai.model` | Modelo a usar | Sí* |
| `profiles[].ai.base_url` | URL base (para Ollama local) | No |
| `profiles[].repos` | Lista de repositorios | Sí |
| `repos[].provider` | Provider: `github`, `gitlab`, `bitbucket` | Sí |
| `repos[].owner` | Owner/namespace del repo | Sí |
| `repos[].name` | Nombre del repositorio | Sí |

*Solo obligatorio si usas IA

### Providers de IA

| Provider | Modelos recomendados | Notas |
|----------|---------------------|-------|
| `ollama` | `llama3`, `mistral`, `ministral-3:8b` | Requiere Ollama corriendo localmente |
| `openai` | `gpt-4o`, `gpt-4o-mini` | Requiere API key de OpenAI |
| `anthropic` | `claude-3-5-sonnet-20241022` | Requiere API key de Anthropic |

## 🖥️ Uso

```bash
# Generar reporte de las últimas 2 semanas (por defecto)
./git-reports

# Especificar período
./git-reports -p week      # última semana
./git-reports -p 2weeks    # últimas 2 semanas (default)
./git-reports -p month     # último mes
./git-reports -p all       # todos los commits

# Especificar config diferente
./git-reports -c mi-config.yaml

# Cambiar nombre del reporte
./git-reports -n mi-reporte
```

### Opciones del CLI

| Opción | Descripción | Default |
|--------|-------------|---------|
| `-c, --config` | Archivo de configuración | `config.yaml` |
| `-o, --output-dir` | Directorio de salida | `reports` |
| `-n, --name` | Nombre base del reporte | `report` |
| `-p, --period` | Período: `week`, `2weeks`, `month`, `all` | `2weeks` |
| `-h, --help` | Mostrar ayuda | - |

### Salida

El ejecutable crea una carpeta `reports/` con:

```
reports/
├── report_2026-02-12_2026-02-26.json
└── report_2026-02-12_2026-02-26.md
```

- **JSON**: Datos crudos con todos los commits
- **Markdown**: Reporte formateado estilo Jira con tareas

## 📊 Ejemplo de reporte

El reporte Markdown incluye:

- **Tareas estilo Jira**: Título, descripción, tiempo estimado, tipo, esfuerzo
- **Resumen por área**: Backend, frontend, testing, etc.
- **40 horas semanales**: Total distribuido entre tareas

Ver [examples/reporte-ejemplo.md](examples/reporte-ejemplo.md) para un ejemplo completo.

## 🛠️ Desarrollo

```bash
# Compilar
cargo build --release

# Tests
cargo test

# Formato
cargo fmt --check
```

## 📄 Licencia

MIT - Ver [LICENSE](LICENSE) para detalles.

---

**Autor**: [madkoding](https://github.com/madkoding)

<!-- AUTO-UPDATE-DATE -->
**Última actualización:** 2026-02-26 14:55:34 -03
