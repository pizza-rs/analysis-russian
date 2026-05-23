//! Russian ё→е normalization.

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use pizza_engine::analysis::{Token, TokenFilter};

/// Normalizes Russian ё/Ё to е/Е.
///
/// Critical for Russian search quality since users interchangeably use both.
#[derive(Clone, Debug, Default)]
pub struct RussianYoFilter;

impl RussianYoFilter {
    pub fn new() -> Self {
        Self
    }
}

impl TokenFilter for RussianYoFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let text = token.term.as_ref();
        if !text.contains('ё') && !text.contains('Ё') {
            return (false, None);
        }
        let normalized: String = text
            .chars()
            .map(|c| match c {
                'ё' => 'е',
                'Ё' => 'Е',
                _ => c,
            })
            .collect();
        token.term = Cow::Owned(normalized);
        (false, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yo_normalization() {
        let f = RussianYoFilter::new();
        let mut token = Token::new("ёлка", 0, 8, 0);
        f.filter(&mut token);
        assert_eq!(token.term, "елка");
    }

    #[test]
    fn test_no_yo() {
        let f = RussianYoFilter::new();
        let mut token = Token::new("привет", 0, 12, 0);
        f.filter(&mut token);
        assert_eq!(token.term, "привет");
    }
}
