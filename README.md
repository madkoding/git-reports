# Git Reports

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

### Uso

```bash
cargo run --release -- --repo /path/to/repo --output report.json
```

## 🛠️ Stack Tecnológico

- **Lenguaje**: Rust
- **Licencia**: MIT
- **Tests**: 100% passing

## 📊 Métricas

![CI](https://img.shields.io/github/actions/workflow/status/madkoding/git-reports/ci.yml?label=CI&logo=github)
![License](https://img.shields.io/github/license/madkoding/git-reports?logo=github)
![Rust](https://img.shields.io/badge/rust-1.70+-DC2626?logo=rust&logoColor=white)

## 📄 Licencia

MIT - Ver [LICENSE](LICENSE) para detalles.

---

**Authored by**: [madkoding](https://github.com/madkoding)

<!-- AUTO-UPDATE-DATE -->
**Última actualización:** 2026-02-20 20:08:43 -03
