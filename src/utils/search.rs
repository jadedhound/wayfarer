use std::cmp;

pub type CharCount = [u8; 36];

pub fn char_count(s: &str) -> CharCount {
    let mut count = [0; 36];
    s.chars().for_each(|c| {
        count[constrain_index(c as u8)] += 1;
    });
    count
}

pub const fn compile_count(s: &str) -> CharCount {
    recurse_count(s, [0; 36], 0)
}

const fn recurse_count(s: &str, mut acc: CharCount, i: usize) -> CharCount {
    if i < s.len() {
        let index = constrain_index(s.as_bytes()[i]);
        acc[index] += 1;
        recurse_count(s, acc, i + 1)
    } else {
        acc
    }
}

/// Maps char onto an constrained index ranging from 0 to 35.
const fn constrain_index(i: u8) -> usize {
    // '0' is 48 and '9' is 57. Index of 0 - 9 for numbers.
    let result = if i >= 48 && i <= 57 {
        i - 48
    }
    // 'a' is 97 and 'z' is 122. Index of 10 - 34 for letters.
    else if i >= 97 && i <= 122 {
        i - 97
    // Default to last index for everything else.
    } else {
        35
    };
    result as usize
}

/// Whether `pattern` is in `text` by a given `confidence` from 0 (0% match) to 100 (100% match).
pub fn contains_pattern(pattern: &CharCount, text: &CharCount, confidence: u8) -> bool {
    // Start as a complete match.
    let mut curr_confidence = 100;
    // Each miss degrades the `curr_confidence` by % based on number of chars.
    let miss_step = cmp::max(100 / pattern.iter().sum::<u8>(), 1);
    let pattern_values_only = pattern.iter().enumerate().filter(|(_, x)| x > &&0);

    // Go through every letter counted in pattern. Degrade the confidence
    // if the text is missing letters.
    for (i, p_count) in pattern_values_only {
        let text_count = text[i];
        if text_count > 0 {
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

// -----------------------------------
// HELPERS
// -----------------------------------

pub const fn arr_len<T>(arr: &[&[T]], acc: usize, i: usize) -> usize {
    if i < arr.len() {
        arr_len(arr, acc + arr[i].len(), i + 1)
    } else {
        acc
    }
}
