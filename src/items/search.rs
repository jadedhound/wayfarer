use super::simple::{meta, sundry};
use super::{alchemy, consumables, food, tome, weapons, ItemRef};
use crate::utils::search::{char_count, compile_count, contains_pattern, count_arr, CharCount};

/// Searches through a compiled list of `ItemRef`.
pub fn search(query: &str) -> impl Iterator<Item = &'static ItemRef> {
    let mut results = Vec::new();
    let pattern = char_count(&query.to_lowercase());

    for (text, buff) in SEARCHABLE_ARR {
        if contains_pattern(&pattern, &text, 80) {
            // Absolute difference between the text length and pattern length.
            // Used to sort the list from closest match to least likely.
            let len_diff = usize::abs_diff(buff.name.len(), query.len());
            results.push((buff, len_diff));
        }
    }

    // Sort by length closest to the pattern.
    results.sort_unstable_by(|(_, a_len), (_, b_len)| a_len.cmp(b_len));
    results.into_iter().map(|(x, _)| x)
}

// -----------------------------------
// SEARCH COMPILER
// -----------------------------------

const ALL_ITEMS: [&[&ItemRef]; 9] = [
    &sundry::ALL,
    &meta::ALL,
    &alchemy::t1::ALL,
    &tome::spell::ALL,
    &tome::prayer::ALL,
    &consumables::BOMB,
    &food::ALL,
    &weapons::ALL,
    &consumables::misc::ALL,
];

const TOTAL_ITEMS: usize = count_arr(&ALL_ITEMS, 0, 0);
type CountedArr = [(CharCount, &'static ItemRef); TOTAL_ITEMS];
const COUNTED_ARR_DEF: CountedArr = [([0; 36], &meta::ERROR_ITEM); TOTAL_ITEMS];
const SEARCHABLE_ARR: CountedArr = compile_search(&ALL_ITEMS, 0, COUNTED_ARR_DEF, 0);

/// Recursively compile `CountedArr` to be searched.
const fn compile_search(
    arr: &[&[&'static ItemRef]],
    arr_i: usize,
    acc: CountedArr,
    acc_i: usize,
) -> CountedArr {
    if arr_i < arr.len() {
        let (acc, acc_i) = compile_subsearch(acc, acc_i, arr[arr_i], 0);
        compile_search(arr, arr_i + 1, acc, acc_i)
    } else {
        acc
    }
}

/// Recursively got through each BuffRef array and create a `CountedArr` for each.
const fn compile_subsearch(
    mut acc: CountedArr,
    acc_i: usize,
    list: &[&'static ItemRef],
    list_i: usize,
) -> (CountedArr, usize) {
    if list_i < list.len() {
        let char_count = compile_count(list[list_i].name);
        acc[acc_i + list_i] = (char_count, list[list_i]);
        compile_subsearch(acc, acc_i, list, list_i + 1)
    } else {
        // Adjust the current position of the `acc_i` by the amount of
        // items we just added.
        (acc, acc_i + list_i)
    }
}
