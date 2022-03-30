use crate::debug::annotation::DisplaySnippet;
use annotate_snippets::display_list::DisplayList;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn print_snippet<T: DisplaySnippet>(
    snippet: T,
    src: &str,
    lineno: usize,
    origin: Option<&str>,
) {
    let dl = DisplayList::from(snippet.as_snippet(src, lineno, origin));
    println!("{}", dl);
}

/// Internally, u64s are used since they can be compared much faster than strings
pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
