// use crate::debug::diagnostic::Diagnostic;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Internally, u64s are used since they can be compared much faster than strings
pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
