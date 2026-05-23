//! Russian stop words (from Lucene/Snowball project).

use alloc::borrow::Cow;
use alloc::vec::Vec;
use hashbrown::HashSet;
use once_cell::sync::Lazy;
use pizza_engine::analysis::{Token, TokenFilter};

/// Default Russian stop words sourced from Apache Lucene.
static DEFAULT_STOP_WORDS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let words: &[&str] = &[
    "а",
    "без",
    "более",
    "больше",
    "будет",
    "будто",
    "бы",
    "был",
    "была",
    "были",
    "было",
    "быть",
    "в",
    "вам",
    "вас",
    "вдруг",
    "ведь",
    "во",
    "вот",
    "впрочем",
    "все",
    "всегда",
    "всего",
    "всех",
    "всю",
    "вы",
    "где",
    "говорил",
    "да",
    "даже",
    "два",
    "для",
    "до",
    "другой",
    "его",
    "ее",
    "ей",
    "ему",
    "если",
    "есть",
    "еще",
    "ж",
    "же",
    "жизнь",
    "за",
    "зачем",
    "здесь",
    "и",
    "из",
    "или",
    "им",
    "иногда",
    "их",
    "к",
    "кажется",
    "как",
    "какая",
    "какой",
    "когда",
    "конечно",
    "кто",
    "куда",
    "ли",
    "лучше",
    "между",
    "меня",
    "мне",
    "много",
    "может",
    "можно",
    "мой",
    "моя",
    "мы",
    "на",
    "над",
    "надо",
    "наконец",
    "нас",
    "не",
    "него",
    "нее",
    "ней",
    "нельзя",
    "нет",
    "ни",
    "нибудь",
    "никогда",
    "ним",
    "них",
    "ничего",
    "но",
    "ну",
    "о",
    "об",
    "один",
    "он",
    "она",
    "они",
    "опять",
    "от",
    "перед",
    "по",
    "под",
    "после",
    "потом",
    "потому",
    "почти",
    "при",
    "про",
    "раз",
    "разве",
    "с",
    "сам",
    "свою",
    "себе",
    "себя",
    "сегодня",
    "сейчас",
    "сказал",
    "сказала",
    "сказать",
    "со",
    "совсем",
    "так",
    "такой",
    "там",
    "тебя",
    "тем",
    "теперь",
    "то",
    "тогда",
    "того",
    "тоже",
    "только",
    "том",
    "тот",
    "три",
    "тут",
    "ты",
    "у",
    "уж",
    "уже",
    "хорошо",
    "хоть",
    "чего",
    "человек",
    "чем",
    "через",
    "что",
    "чтоб",
    "чтобы",
    "чуть",
    "эти",
    "этого",
    "этой",
    "этом",
    "этот",
    "эту",
    "я",
    ];
    words.iter().copied().collect()
});

/// Removes Russian stop words from the token stream.
#[derive(Clone, Debug)]
pub struct RussianStopFilter {
    stop_words: HashSet<String>,
}

impl Default for RussianStopFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl RussianStopFilter {
    pub fn new() -> Self {
        Self {
            stop_words: DEFAULT_STOP_WORDS.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn with_words(words: &[&str]) -> Self {
        Self {
            stop_words: words.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl TokenFilter for RussianStopFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let term = token.term.as_ref();
        if self.stop_words.contains(term) {
            return (true, None);
        }
        (false, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stop_word_count() {
        assert!(DEFAULT_STOP_WORDS.len() >= 159);
    }

    #[test]
    fn test_filters_stop_word() {
        let f = RussianStopFilter::new();
        let word = DEFAULT_STOP_WORDS.iter().next().unwrap();
        let mut token = Token::new(word, 0, word.len() as u32, 0);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted);
    }

    #[test]
    fn test_passes_non_stop_word() {
        let f = RussianStopFilter::new();
        let mut token = Token::new("xyzzy_not_a_stop_word", 0, 21, 0);
        let (deleted, _) = f.filter(&mut token);
        assert!(!deleted);
    }

    #[test]
    fn test_custom_words() {
        let f = RussianStopFilter::with_words(&["custom", "words"]);
        let mut token = Token::new("custom", 0, 6, 0);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted);
    }
}
