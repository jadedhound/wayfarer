use std::collections::HashMap;
use std::str::Split;

use once_cell::sync::Lazy;
use strum::IntoEnumIterator;

use super::fuzzy_match::*;
use super::{Armour, Gem, Held, Item, GEMS, RUNES, SUNDRIES};
use crate::items::Gear;

/// Convenience fn to generate a string comparison hash from
/// an existing hash.
fn hash_to_cmp<T>(hash: &Lazy<HashMap<String, T>>) -> Vec<(String, CharVec)> {
    hash.iter()
        .map(|(name, _)| (name.to_owned(), create_char_vec(name)))
        .collect()
}

fn enum_to_cmp<D>(d: D) -> (String, CharVec)
where
    D: std::fmt::Display,
{
    let d = d.to_string();
    let d_cmp = create_char_vec(&d);
    (d, d_cmp)
}

pub static SUNDRY_CMP: LazyCmp = Lazy::new(|| hash_to_cmp(&SUNDRIES));

pub static HELD_TYPES: LazyCmp = Lazy::new(|| Held::iter().map(enum_to_cmp).collect());

pub static ARMOUR_TYPES: LazyCmp = Lazy::new(|| Armour::iter().map(enum_to_cmp).collect());

pub static GEMS_CMP: LazyCmp = Lazy::new(|| hash_to_cmp(&GEMS));

pub static RUNES_CMP: LazyCmp = Lazy::new(|| hash_to_cmp(&RUNES));

#[rustfmt::skip]
const FATIGUE: [(char, f32); 7] = [('f', 1.0),('a', 1.0),('t', 1.0),('i', 1.0),('g', 1.0),('u', 1.0),('e', 1.0)];

fn is_gem(s: &CharVec) -> Option<Gem> {
    fuzzy_match(s, &GEMS_CMP, 0.8).and_then(|g| GEMS.get(&g).cloned())
}

fn is_rune(words: &mut Split<char>) -> Option<String> {
    let end_words = words.fold(String::new(), |acc, e| acc + e);
    let maybe_rune = create_char_vec(&end_words);
    fuzzy_match(&maybe_rune, &RUNES_CMP, 0.8)
}

fn is_held(words: &mut Split<char>, s: &CharVec, quality: u8) -> Option<Item> {
    let h = fuzzy_match(s, &HELD_TYPES, 0.7)?;
    let held = Gear::create(None, h, is_rune(words), quality)?;
    Some(Item::Held(held))
}

fn is_armour(words: &mut Split<char>, s: &CharVec, quality: u8) -> Option<Item> {
    let h = fuzzy_match(s, &ARMOUR_TYPES, 0.7)?;
    let armour = Gear::create(None, h, is_rune(words), quality)?;
    Some(Item::Armour(armour))
}

fn is_sundry(s: &CharVec, _quality: u8) -> Option<Item> {
    let h = fuzzy_match(s, &SUNDRY_CMP, 0.4)?;
    let sundry = SUNDRIES.get(&h).cloned()?;
    Some(Item::Simple(sundry))
}

fn is_fatigue(s: &CharVec) -> Option<Item> {
    if compare_letters(s, &FATIGUE.to_vec()) > 0.7 {
        Some(Item::Fatigue)
    } else {
        None
    }
}

pub fn search(s: String, quality: u8) -> Option<Item> {
    let mut words = s.split(' ');
    let first = create_char_vec(words.next()?);

    // Check if fatigue > held > armour > sundry
    is_fatigue(&first)
        .or_else(|| is_held(&mut words, &first, quality))
        .or_else(|| is_armour(&mut words, &first, quality))
        .or_else(|| {
            let s = create_char_vec(&s);
            is_sundry(&s, quality)
        })
}
