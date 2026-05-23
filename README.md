<div align="center">

# 🇷🇺 pizza-analysis-russian

**Russian text analysis plugin for [INFINI Pizza](https://pizza.rs)**

[![Crate](https://img.shields.io/badge/crate-pizza--analysis--russian-blue)](https://github.com/pizza-rs/analysis-russian)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)

</div>

---

## Overview

Production-ready Russian language analysis with character normalization, light stemming,
and stop word removal. Designed for Russian full-text search with correct handling of
the ё/е alternation common in Russian text.

## Components

| Type | Name | Description |
|:-----|:-----|:------------|
| TokenFilter | `russian_yo` | Normalize ё→е / Ё→Е (common spelling variation) |
| TokenFilter | `russian_light_stem` | Light suffix-stripping stemmer for Russian |
| TokenFilter | `russian_stop` | Russian stop words (159 entries) |
| Analyzer | `russian` | Full pipeline: lowercase → russian_yo → light_stem → stop |

### Russian Yo Normalization

In modern Russian text, ё (yo) is frequently written as е (ye). This filter
normalizes both forms for consistent matching:

- `ещё` → `еще`
- `Ёлка` → `Елка`

### Light Stemmer

Removes common Russian suffixes without aggressive over-stemming. Handles
noun/adjective/verb endings while preserving the stem for high-precision matching.

## Example

```rust
use pizza_engine::analysis::AnalysisFactory;

let mut factory = AnalysisFactory::new();
pizza_analysis_russian::register_all(&mut factory);

let analyzer = factory.get_analyzer("russian").unwrap();
// "бегущие собаки" → tokens: ["бегущ", "собак"]
```

## Installation

```toml
[dependencies]
pizza-analysis-russian = "0.1"
```

Or via `pizza-analysis-all`:

```toml
[dependencies]
pizza-analysis-all = { version = "0.1", features = ["russian"] }
```

## License

MIT

---

<div align="center">
<sub>Part of the <a href="https://pizza.rs">INFINI Pizza</a> ecosystem</sub>
</div>
