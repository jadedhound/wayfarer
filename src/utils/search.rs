/// Searches through a compiled list of `BuffRef`.
pub fn search<T: Copy, P>(
    arr: &[&[&'static T]],
    pattern: P,
    query: String,
) -> impl IntoIterator<Item = &'static T>
where
    P: Fn(&'static T) -> &str,
{
    let query = query.to_lowercase();
    let smaller_than = |name_len: usize, other: Option<&'static T>| {
        name_len < other.map(|ele| pattern(ele).len()).unwrap_or(usize::MAX)
    };
    let mut small = None;
    let mut mid = None;
    let mut large = None;
    let filter = arr.iter().flat_map(|arr| {
        arr.iter().filter(|ele| {
            let mut pattern = pattern(ele).to_owned();
            // Remove any special characters from the pattern.
            pattern.retain(|c| !r#"(),".;:'"#.contains(c));
            pattern.contains(&query)
        })
    });
    for ele in filter {
        let name_len = pattern(ele).len();
        if smaller_than(name_len, small) {
            // Move everything else down.
            large = mid;
            mid = small;
            small = Some(ele);
        } else if smaller_than(name_len, mid) {
            large = mid;
            mid = Some(ele)
        } else if smaller_than(name_len, large) {
            large = Some(ele)
        }
    }
    [small, mid, large].into_iter().flatten()
}
