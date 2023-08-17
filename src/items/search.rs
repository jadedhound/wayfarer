use std::collections::HashMap;
use std::fmt::Display;

use once_cell::sync::Lazy;
use serde::Deserialize;
use serde::Serialize;
use strum::IntoEnumIterator;

use crate::tables::forage::ForageTable;

use super::consumables;
use super::food;
use super::potions;
use super::reagents;
use super::simple;
use super::tome;
use super::weapons;
use super::{Item, ItemRef};

type CharCount = Vec<(char, f32)>;
type LazyCmp = Lazy<Vec<(&'static ItemRef, CharCount)>>;
type ResultCmp = Lazy<Vec<(SearchResult, CharCount)>>;

#[derive(Serialize, Deserialize, Clone)]
pub enum SearchResult {
    Forage(ForageTable),
    Item(Item),
}

impl SearchResult {
    pub fn to_item(&self) -> Item {
        match self {
            Self::Forage(x) => x.to_item(),
            Self::Item(x) => x.clone(),
        }
    }
    fn deref_str(&self) -> &str {
        match self {
            SearchResult::Forage(x) => x.as_ref(),
            SearchResult::Item(_) => "",
        }
    }
}

impl Display for SearchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SearchResult::Forage(x) => write!(f, "forage {}", x.as_ref()),
            SearchResult::Item(x) => write!(f, "{}", x.name),
        }
    }
}

const EVERY_ITEM: [&[&ItemRef]; 8] = [
    &weapons::ALL,
    &reagents::ALL,
    &simple::SUNDRIES,
    &simple::META,
    &potions::ALL,
    &tome::ALL,
    &consumables::BOMB,
    &food::ALL,
];

static ITEMS_CMP: LazyCmp = Lazy::new(|| {
    let mut cmp = Vec::new();
    for item_arr in EVERY_ITEM {
        for item in item_arr {
            let count = create_char_count(item.name);
            cmp.push((*item, count));
        }
    }
    cmp
});
static FORAGE_CMP: ResultCmp = Lazy::new(|| {
    let mut cmp = Vec::new();
    for table in ForageTable::iter() {
        let count = create_char_count(table.as_ref());
        cmp.push((SearchResult::Forage(table), count));
    }
    cmp
});

pub fn create_char_count(s: &str) -> CharCount {
    let mut hash = HashMap::new();
    s.chars().for_each(|c| {
        hash.entry(c)
            .and_modify(|counter| *counter += 1.0)
            .or_insert(1.0);
    });
    hash.into_iter().collect()
}

/// Whether `pattern` matches `text` by a given `confidence` percentage.
pub fn contains_pattern(pattern: &CharCount, text: &CharCount, confidence: f32) -> bool {
    fn char_to_index(c: char) -> usize {
        let i = c as usize;
        // 'a' is 97 and 'z' is 122. If the values are not between
        // these, then default to index 25.
        if !(97..=122).contains(&i) {
            25
        } else {
            i - 97
        }
    }
    // Start as a complete match.
    let mut curr_confidence = 1.0;
    // Each miss degrades the `curr_confidence` by % based on number of chars.
    let miss_step = 1.0 / pattern.iter().fold(0.0, |acc, e| acc + e.1);
    let mut char_vecmap = vec![0.0; 26];
    for (c, count) in text {
        char_vecmap[char_to_index(*c)] = *count
    }

    // Go through every letter counted in pattern. Degrade the confidence
    // if the text is missing letters.
    for (p_letter, p_count) in pattern {
        let text_count = char_vecmap[char_to_index(*p_letter)];
        if text_count > 0.0 {
            if p_count > &text_count {
                curr_confidence -= miss_step * (p_count - text_count);
                if curr_confidence < confidence {
                    return false;
                }
            }
        } else {
            curr_confidence -= miss_step * p_count;
            if curr_confidence < confidence {
                return false;
            }
        }
    }

    true
}

/// Searches all items for a given `query`.
fn search_items(query: &str) -> Vec<(SearchResult, usize)> {
    const CONFIDENCE: f32 = 0.8;

    let pattern = create_char_count(query);
    let pattern_len = query.len();
    let mut results = Vec::new();
    ITEMS_CMP.iter().for_each(|(item, text)| {
        if contains_pattern(&pattern, text, CONFIDENCE) {
            let x = SearchResult::Item((**item).into());
            // Needed for sorting later.
            let x_len = usize::abs_diff(x.to_string().len(), pattern_len);
            results.push((x, x_len))
        }
    });
    results
}

fn search_tables(query: &str, tables: &ResultCmp) -> Vec<(SearchResult, usize)> {
    const CONFIDENCE: f32 = 0.8;

    let pattern = create_char_count(query);
    let pattern_len = query.len();

    let mut results = Vec::new();
    tables.iter().for_each(|(result, text)| {
        if contains_pattern(&pattern, text, CONFIDENCE) {
            // Needed for sorting later.
            let result_len = usize::abs_diff(result.deref_str().len(), pattern_len);
            results.push((result.clone(), result_len))
        }
    });
    results
}

/// Search.
pub fn search(query: &str) -> impl Iterator<Item = SearchResult> {
    let mut words_arr = query.split(' ');
    let mut results = if let Some(word) = words_arr.next() {
        match word {
            "forage" => search_tables(&words_arr.collect::<String>(), &FORAGE_CMP),
            _ => search_items(query),
        }
    } else {
        search_items(query)
    };

    // Sort by length closest to the pattern.
    results.sort_unstable_by(|(_, a_len), (_, b_len)| a_len.cmp(b_len));
    results.into_iter().map(|(x, _)| x)
}
