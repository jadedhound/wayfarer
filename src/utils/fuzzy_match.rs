use std::collections::HashMap;

use once_cell::sync::Lazy;

pub type CharHash = HashMap<char, f32>;

/// Convenience fn to generate a string comparison hash from
/// an existing hash.
pub fn cmphash_from_hash<T>(hash: &Lazy<HashMap<String, T>>) -> HashMap<String, CharHash> {
    hash.iter()
        .map(|(name, _)| (name.to_owned(), char_count(name)))
        .collect()
}

pub fn char_count_tuple<D>(d: D) -> (String, CharHash)
where
    D: std::fmt::Display,
{
    let d = d.to_string();
    let d_counted = char_count(&d);
    (d, d_counted)
}

pub fn char_count(s: &str) -> HashMap<char, f32> {
    let mut hash = HashMap::new();
    s.chars().for_each(|c| {
        hash.entry(c)
            .and_modify(|counter| *counter += 1.0)
            .or_insert(1.0);
    });
    hash
}

fn char_match(cmp_hash: &CharHash, ref_hash: &CharHash) -> f32 {
    let mut hit_count = 0.0;
    let mut miss_count = 0.0;
    cmp_hash.iter().for_each(|(c, cmp_am)| {
        if let Some(ref_am) = ref_hash.get(c) {
            if ref_am == cmp_am {
                hit_count += cmp_am
            } else if ref_am > cmp_am {
                hit_count += cmp_am;
                miss_count += ref_am - cmp_am
            } else {
                hit_count += ref_am;
                miss_count += cmp_am - ref_am
            }
        } else {
            miss_count += cmp_am;
        }
    });
    hit_count / (hit_count + miss_count)
}

/// Returns a string from a list that most closely matches a given `x`
/// Higher `confidence` means stricter matching. `x` and `list` have to
/// be of the same case.
pub fn fuzzy_match(
    x: &str,
    hash_list: &Lazy<HashMap<String, CharHash>>,
    confidence: f32,
) -> Option<String> {
    let x_hash = char_count(x);
    let mut val = None;
    hash_list.iter().for_each(|(word, word_hash)| {
        let curr_perc = char_match(word_hash, &x_hash);
        if curr_perc > confidence {
            if let Some((old_perc, _)) = val {
                if curr_perc > old_perc {
                    val = Some((curr_perc, word.to_string()))
                }
            } else {
                val = Some((curr_perc, word.to_string()))
            }
        }
    });
    val.map(|(_, a)| a)
}
