use std::collections::HashMap;

use once_cell::sync::Lazy;

use super::consumables::ITEMS_BOMB;
use super::food::ITEMS_FOOD;
use super::potions::ITEMS_POTION;
use super::reagents::ITEMS_REAGENT;
use super::simple::{ITEMS_META, ITEMS_SIMPLE};
use super::tome::ITEMS_SPELLBOOK;
use super::weapons::ITEMS_WEAP;
use super::{Item, ItemRef};

type CharCount = Vec<(char, f32)>;
type LazyCmp = Lazy<Vec<(&'static ItemRef, CharCount)>>;

const ITEM_ARR: [&[&ItemRef]; 8] = [
    &ITEMS_WEAP,
    &ITEMS_REAGENT,
    &ITEMS_META,
    &ITEMS_POTION,
    &ITEMS_SIMPLE,
    &ITEMS_SPELLBOOK,
    &ITEMS_BOMB,
    &ITEMS_FOOD,
];

static ITEMS_CMP: LazyCmp = Lazy::new(|| {
    let mut cmp = Vec::new();
    for item_arr in ITEM_ARR {
        for item in item_arr {
            let count = create_char_vec(item.name);
            cmp.push((*item, count));
        }
    }
    cmp
});

pub fn create_char_vec(s: &str) -> CharCount {
    let mut hash = HashMap::new();
    s.chars().for_each(|c| {
        hash.entry(c)
            .and_modify(|counter| *counter += 1.0)
            .or_insert(1.0);
    });
    hash.into_iter().collect()
}

/// Compares two `CharVec` and gives a percentage of how close they are
/// as a match
pub fn compare_letters(a: &CharCount, b: &CharCount) -> f32 {
    let mut hit_count = 0.0;
    let mut miss_count = 0.0;
    let (big, sml) = if a.len() > b.len() { (a, b) } else { (b, a) };
    let sml_max_i = sml.iter().fold(0, |prev, (c, _)| {
        let next = *c as usize;
        if next > prev {
            next
        } else {
            prev
        }
    });
    let mut sml_expand = vec![0.0; sml_max_i + 1];
    sml.iter()
        .for_each(|(c, c_count)| sml_expand[*c as usize] = *c_count);

    // Use the bigger of the two, to ensure that no letters are missed
    for (l, big_count) in big.iter() {
        if let Some(sml_count) = sml_expand.get(*l as usize) {
            if sml_count == big_count {
                hit_count += big_count
            } else if sml_count > big_count {
                hit_count += big_count;
                miss_count += sml_count - big_count
            } else {
                hit_count += sml_count;
                miss_count += big_count - sml_count
            }
        } else {
            miss_count += big_count;
        }
    }

    hit_count / (hit_count + miss_count)
}

/// Returns a string from a list that most closely matches a given `x`
/// Higher `confidence` means stricter matching. `x` and `list` have to
/// be of the same case.
pub fn search(query: &str, confidence: f32) -> Option<Item> {
    let query = create_char_vec(query);
    let mut val = None;
    ITEMS_CMP.iter().for_each(|(item, count)| {
        let curr_perc = compare_letters(&query, count);
        if curr_perc > confidence {
            if let Some((perc, _)) = val {
                if curr_perc > perc {
                    val = Some((curr_perc, item))
                }
            } else {
                val = Some((curr_perc, item))
            }
        }
    });
    val.map(|(_, a)| (**a).into())
}
