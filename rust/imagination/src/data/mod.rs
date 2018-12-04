
mod types;

use self::types::*;

// type LinkSV = Option<Box<StatementValues>>;
// type StatementValues = (Value, Value, Value, LinkSV);

struct Statement {
    relation: String,
    value: Vec<Value>,
}

use std::hash::Hash;
use std::collections::{HashMap, HashSet};
struct Index<K: Hash + Eq, V: Hash + Eq>(HashMap<K, HashSet<V>>);

struct Database {
    // indexers: 
}
