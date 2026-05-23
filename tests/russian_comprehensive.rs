//! Comprehensive tests for pizza-analysis-russian.

use pizza_analysis_russian::*;
use pizza_engine::analysis::{AnalysisFactory, Token, TokenFilter};

fn make_token(term: &str) -> Token<'_> {
    Token::new(term, 0, term.len() as u32, 0)
}

// ═══════════════════════════════════════════════════════════════════════════════
// RussianYoFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn yo_filter_construction() {
    let _f = RussianYoFilter::new();
}

#[test]
fn yo_filter_lowercase_yo() {
    let f = RussianYoFilter::new();
    // "ёлка" → "елка"
    let mut token = make_token("ёлка");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert_eq!(token.term.as_ref(), "елка");
}

#[test]
fn yo_filter_uppercase_yo() {
    let f = RussianYoFilter::new();
    // "Ё" → "Е"
    let mut token = make_token("Ёж");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert_eq!(token.term.as_ref(), "Еж");
}

#[test]
fn yo_filter_no_yo() {
    let f = RussianYoFilter::new();
    let mut token = make_token("книга");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert_eq!(token.term.as_ref(), "книга");
}

#[test]
fn yo_filter_ascii_passthrough() {
    let f = RussianYoFilter::new();
    let mut token = make_token("hello");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert_eq!(token.term.as_ref(), "hello");
}

#[test]
fn yo_filter_empty_string() {
    let f = RussianYoFilter::new();
    let mut token = make_token("");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

// ═══════════════════════════════════════════════════════════════════════════════
// RussianLightStemFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn stem_construction() {
    let _f = RussianLightStemFilter::new();
}

#[test]
fn stem_noun_plural() {
    let f = RussianLightStemFilter::new();
    // "книги" (books) → stem
    let mut token = make_token("книги");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_noun_genitive() {
    let f = RussianLightStemFilter::new();
    // "дома" (house, genitive) → stem
    let mut token = make_token("дома");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_adjective_feminine() {
    let f = RussianLightStemFilter::new();
    // "красивая" (beautiful, f.) → stem
    let mut token = make_token("красивая");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_adjective_plural() {
    let f = RussianLightStemFilter::new();
    // "большие" (big, pl.) → stem
    let mut token = make_token("большие");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_verb_past() {
    let f = RussianLightStemFilter::new();
    // "писал" (wrote) → stem
    let mut token = make_token("писал");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_short_word() {
    let f = RussianLightStemFilter::new();
    let mut token = make_token("и");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_empty_string() {
    let f = RussianLightStemFilter::new();
    let mut token = make_token("");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

// ═══════════════════════════════════════════════════════════════════════════════
// RussianStopFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn stop_construction() {
    let _f = RussianStopFilter::new();
}

#[test]
fn stop_filters_common_words() {
    let f = RussianStopFilter::new();
    let stop_words = ["и", "в", "не", "на", "с", "что", "он", "как", "но", "по"];
    for word in &stop_words {
        let mut token = make_token(word);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted, "stop word '{}' should be filtered", word);
    }
}

#[test]
fn stop_keeps_content_words() {
    let f = RussianStopFilter::new();
    let content_words = ["книга", "дом", "школа", "город"];
    for word in &content_words {
        let mut token = make_token(word);
        let (deleted, _) = f.filter(&mut token);
        assert!(!deleted, "content word '{}' should be kept", word);
    }
}

#[test]
fn stop_empty_string() {
    let f = RussianStopFilter::new();
    let mut token = make_token("");
    let _ = f.filter(&mut token);
}

// ═══════════════════════════════════════════════════════════════════════════════
// Registration
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn register_all_no_panic() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
}

#[test]
fn register_all_filters_present() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_token_filter("russian_yo").is_some());
    assert!(factory.get_token_filter("russian_light_stem").is_some());
    assert!(factory.get_token_filter("russian_stop").is_some());
}

#[test]
fn register_all_analyzer_present() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_analyzer("russian").is_some());
}

#[test]
fn analyzer_pipeline_produces_tokens() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("russian").unwrap();
    let mut input = String::from("Дом большой и красивый");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(!tokens.is_empty());
}

#[test]
fn analyzer_pipeline_removes_stops() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("russian").unwrap();
    let mut input = String::from("книга в доме и школе");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    let terms: Vec<&str> = tokens.iter().map(|t| t.term.as_ref()).collect();
    assert!(!terms.contains(&"в"));
    assert!(!terms.contains(&"и"));
}

#[test]
fn analyzer_pipeline_yo_normalization() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("russian").unwrap();
    let mut input = String::from("ёлка");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(!tokens.is_empty());
    // After yo normalization, should not contain ё
    for t in &tokens {
        assert!(!t.term.contains('ё'), "yo should be normalized to е");
    }
}

#[test]
fn analyzer_pipeline_empty_input() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("russian").unwrap();
    let mut input = String::from("");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(tokens.is_empty());
}

#[test]
fn analyzer_pipeline_ascii_input() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("russian").unwrap();
    let mut input = String::from("hello world");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(!tokens.is_empty());
}
