# Git Reports

[![CI](https://img.shields.io/github/actions/workflow/status/madkoding/git-reports/ci.yml?label=CI)](https://github.com/madkoding/git-reports/actions)
[![Release](https://img.shields.io/github/v/release/madkoding/git-reports?label=Version)](https://github.com/madkoding/git-reports/releases)
[![Rust](https://img.shields.io/badge/Rust-1.70+-DC2626?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License](https://img.shields.io/github/license/madkoding/git-reports?label=License)](https://github.com/madkoding/git-reports/blob/main/LICENSE)

Generador de reportes automatizados para repositorios Git. Análisis de commits, contribuidores y métricas de actividad.

## 📋 Descripción

Git Reports es una herramienta construida en Rust que analiza repositorios Git y genera reportes automatizados con:
- Estadísticas de commits por periodo
- Análisis de contribuidores activos
- Métricas de actividad y patrones
- Exportación en múltiples formatos

## 🎯 Estado del Proyecto

**Milestone Actual**: 50% Complete

| Característica | Estado |
|---------------|--------|
| Análisis de commits | ✅ |
| Métricas de contribuidores | ✅ |
| Exportación de reportes | 🚧 WIP |
| Configuración multi-perfil (config.toml) | ⏳ Planificado |
| Soporte providers remotos (GitHub/GitLab/Bitbucket) | ⏳ Planificado |
| Visualización web | ⏳ Planificado |

## 🚀 Inicio Rápido

### Prerrequisitos

- Rust 1.70+
- Cargo

### Instalación

```bash
git clone https://github.com/madkoding/git-reports.git
cd git-reports
cargo build --release
```

## ⚙️ Configuración

> **Estado**: ⏳ Planificado — aún no implementado. El diseño de la configuración está definido en [config.toml.example](config.toml.example) como referencia para la implementación futura.

Git Reports trabajará con perfiles: cada uno agrupa un email, un token y una lista de repositorios remotos. Esto permite manejar varias empresas, providers y cuentas desde un único archivo.

### Archivo de configuración

```bash
cp config.toml.example config.toml
```

Estructura de `config.toml`:

```toml
[[profile]]
name    = "trabajo"
email   = "yo@empresa.com"
token   = "ghp_xxxxxxxxxxxxxxxxxxxx"   # Personal Access Token del provider

  [[profile.repo]]
  provider = "github"     # github | gitlab | bitbucket
  owner    = "mi-empresa"
  name     = "mi-repo"
```

### Múltiples empresas, correos y tokens

Cada `[[profile]]` es una identidad independiente. Puedes tener tantos como necesites:

| Situación | Solución |
|---|---|
| Varias empresas | Un perfil por empresa, cada uno con su email y token |
| Mismo correo en GitHub y Bitbucket | Un perfil por provider, con su propio token |
| Freelance + trabajo + personal | Un perfil por contexto |

```toml
[[profile]]
name  = "empresa-a"
email = "yo@empresa-a.com"
token = "ghp_..."          # GitHub PAT de empresa-a
  [[profile.repo]]
  provider = "github"
  owner = "empresa-a"
  name  = "backend-api"

[[profile]]
name  = "empresa-b"
email = "yo@empresa-b.com"
token = "glpat-..."        # GitLab PAT de empresa-b
  [[profile.repo]]
  provider = "gitlab"
  owner = "empresa-b"
  name  = "infra-scripts"
```

Ver [config.toml.example](config.toml.example) para un ejemplo completo con todos los casos.

> ⚠️ Agrega `config.toml` a tu `.gitignore` para no exponer tokens.

## 🖥️ Uso

> **Estado actual**: solo analiza repositorios locales ya clonados.

```bash
cargo run --release -- --repo /path/to/repo --output report.json
```

| Argumento | Descripción | Default |
|---|---|---|
| `--repo` | Ruta local al repositorio | `.` |
| `--output` | Archivo JSON de salida | — |
| `--period` | Periodo: `week`, `month`, `all` | `week` |

## 🛠️ Stack Tecnológico

- **Lenguaje**: Rust
- **Licencia**: MIT
- **Tests**: 100% passing

## 📊 Métricas

![CI](https://img.shields.io/github/actions/workflow/status/madkoding/git-reports/ci.yml?label=CI&logo=github)
[![Version](https://img.shields.io/github/v/release/madkoding/git-reports?logo=github)](https://github.com/madkoding/git-reports/releases) [![Rust](https://img.shields.io/badge/rust-1.70+-DC2626?logo=rust&logoColor=white)](https://www.rust-lang.org)
![License](https://img.shields.io/github/license/madkoding/git-reports?logo=github)
![Rust](https://img.shields.io/badge/rust-1.70+-DC2626?logo=rust&logoColor=white)

## 📄 Licencia

MIT - Ver [LICENSE](LICENSE) para detalles.

---

**Authored by**: [madkoding](https://github.com/madkoding)

<!-- AUTO-UPDATE-DATE -->
**Última actualización:** 2026-02-25 14:24:56 -03
