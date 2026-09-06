//! Register Russian analysis components into [`AnalysisFactory`].

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

use pizza_engine::analysis::AnalysisFactory;
use pizza_engine::analysis::Analyzer;
use pizza_engine::analysis::LowercaseNormalizer;
use pizza_engine::analysis::Normalizer;
use pizza_engine::analysis::StandardTokenizer;
use pizza_engine::analysis::TokenFilter;
use pizza_engine::analysis::Tokenizer;

use crate::RussianLightStemFilter;
use crate::RussianStopFilter;
use crate::RussianYoFilter;

/// Register Russian token filters and the `"russian"` analyzer.
pub fn register_all(factory: &mut AnalysisFactory) {
    factory.register_token_filter("russian_yo", Box::new(RussianYoFilter::new()));
    factory.register_token_filter(
        "russian_light_stem",
        Box::new(RussianLightStemFilter::new()),
    );
    factory.register_token_filter("russian_stop", Box::new(RussianStopFilter::new()));

    let normalizers: Vec<Box<dyn Normalizer>> = vec![Box::new(LowercaseNormalizer::new())];
    let tokenizer: Box<dyn Tokenizer> = Box::new(StandardTokenizer::new());
    let filters: Vec<Box<dyn TokenFilter>> = vec![
        Box::new(RussianYoFilter::new()),
        Box::new(RussianStopFilter::new()),
        Box::new(RussianLightStemFilter::new()),
    ];
    factory.register_analyzer("russian", Analyzer::new(normalizers, tokenizer, filters));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_all_no_panic() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
    }

    #[test]
    fn test_filters_registered() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        assert!(factory.get_token_filter("russian_yo").is_some());
        assert!(factory.get_token_filter("russian_light_stem").is_some());
        assert!(factory.get_token_filter("russian_stop").is_some());
    }

    #[test]
    fn test_analyzer_registered() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        assert!(factory.get_analyzer("russian").is_some());
    }

    #[test]
    fn test_analyzer_pipeline() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        let analyzer = factory.get_analyzer("russian").unwrap();
        let mut input = String::from("Кошка не в доме");
        let tokens = analyzer.analyze_and_return_tokens(&mut input);
        assert!(!tokens.iter().any(|t| t.term == "не"));
        assert!(!tokens.iter().any(|t| t.term == "в"));
        assert!(tokens.len() >= 1);
    }
}
