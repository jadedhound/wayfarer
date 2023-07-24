use std::collections::HashMap;

use once_cell::sync::Lazy;

pub type LazyCmp = Lazy<Vec<(String, CharVec)>>;

pub type CharVec = Vec<(char, f32)>;

pub fn create_char_vec(s: &str) -> CharVec {
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
pub fn compare_letters(a: &CharVec, b: &CharVec) -> f32 {
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
pub fn fuzzy_match(x: &CharVec, arr: &LazyCmp, confidence: f32) -> Option<String> {
    let mut val = None;
    arr.iter().for_each(|(word, cmp_word)| {
        let curr_perc = compare_letters(x, cmp_word);
        if curr_perc > confidence {
            if let Some((perc, _)) = val {
                if curr_perc > perc {
                    val = Some((curr_perc, word.to_string()))
                }
            } else {
                val = Some((curr_perc, word.to_string()))
            }
        }
    });
    val.map(|(_, a)| a)
}
