//! Russian light stemmer.

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use pizza_engine::analysis::Token;
use pizza_engine::analysis::TokenFilter;

/// Russian light stemmer — removes common Russian suffixes.
#[derive(Clone, Debug, Default)]
pub struct RussianLightStemFilter;

impl RussianLightStemFilter {
    pub fn new() -> Self {
        Self
    }
}

impl TokenFilter for RussianLightStemFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let text = token.term.as_ref();
        let chars: Vec<char> = text.chars().collect();
        if chars.len() < 4 {
            return (false, None);
        }
        if let Some(new_len) = stem_russian_light(&chars) {
            if new_len < chars.len() {
                let stemmed: String = chars[..new_len].iter().collect();
                token.term = Cow::Owned(stemmed);
            }
        }
        (false, None)
    }
}

fn stem_russian_light(chars: &[char]) -> Option<usize> {
    let len = chars.len();

    // 5-char suffixes
    if len > 6 {
        let suffix: String = chars[len - 5..].iter().collect();
        match suffix.as_str() {
            "ейший" | "ейшая" | "ейшее" | "ейшие" => return Some(len - 5),
            _ => {}
        }
    }

    // 4-char suffixes
    if len > 5 {
        let suffix: String = chars[len - 4..].iter().collect();
        match suffix.as_str() {
            "ость" | "ений" | "ения" => return Some(len - 4),
            _ => {}
        }
    }

    // 3-char suffixes
    if len > 4 {
        let suffix: String = chars[len - 3..].iter().collect();
        match suffix.as_str() {
            "ами" | "ями" | "ому" | "ого" | "ним" | "ных" | "ить" | "ать" | "ять" | "ной"
            | "ное" | "ная" | "ные" => return Some(len - 3),
            _ => {}
        }
    }

    // 2-char suffixes
    if len > 3 {
        let suffix: String = chars[len - 2..].iter().collect();
        match suffix.as_str() {
            "ов" | "ев" | "ей" | "ий" | "ая" | "ое" | "ые" | "ам" | "ям" | "ом" | "ем" | "ах"
            | "ях" | "ую" | "юю" | "ть" | "ых" | "их" => return Some(len - 2),
            _ => {}
        }
    }

    // 1-char suffixes
    if len > 3 {
        let last = chars[len - 1];
        match last {
            'а' | 'е' | 'и' | 'о' | 'у' | 'ы' | 'ь' | 'я' | 'й' => return Some(len - 1),
            _ => {}
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_russian_light() {
        let f = RussianLightStemFilter::new();
        let mut token = Token::new("книги", 0, 10, 0);
        f.filter(&mut token);
        assert!(token.term.as_ref().len() < "книги".len());
    }

    #[test]
    fn test_short_word() {
        let f = RussianLightStemFilter::new();
        let mut token = Token::new("да", 0, 4, 0);
        f.filter(&mut token);
        assert_eq!(token.term, "да");
    }
}
