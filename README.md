# pizza-analysis-russian

Russian language analysis with ё→е normalization, light stemmer, and stop words.

Part of the [Pizza](https://pizza.rs) search engine.

## Components

| Name | Type | Description |
|------|------|-------------|
| `russian_yo` | Token Filter | Normalizes ё→е (common Russian spelling variation) |
| `russian_stem` | Token Filter | Russian light stemmer — removes common case/plural/verb suffixes |
| `russian_stop` | Token Filter | Russian stop words filter (159 words) |
| `russian` | Analyzer | Full pipeline: lowercase → yo_normalization → stop → stem |

## Usage

### Built-in Analyzer

```json
{
  "analyzer": {
    "type": "russian"
  }
}
```

### Custom Pipeline

```json
{
  "analyzer": {
    "type": "custom",
    "tokenizer": "standard",
    "filter": ["russian_yo", "russian_stem", "russian_stop"]
  }
}
```

## License

MIT — see [LICENSE](LICENSE).

## Related Crates

- [analysis-core](https://github.com/pizza-rs/analysis-core) — Core analysis components and pipeline
- [analysis-icu](https://github.com/pizza-rs/analysis-icu) — ICU Unicode normalization and tokenization
- [analysis-english](https://github.com/pizza-rs/analysis-english) — English analysis
- [analysis-all](https://github.com/pizza-rs/analysis-all) — Meta-crate registering all analyzers
