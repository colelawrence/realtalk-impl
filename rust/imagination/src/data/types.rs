use std::collections::{HashMap, HashSet};

// address, index, generation
#[derive(Hash, PartialEq, Eq)]
struct Symbol(usize, usize, usize);

#[derive(Hash, PartialEq, Eq)]
pub enum Value {
    Symbol(Symbol),
    Str(String),
    I64(i64),
}

#[derive(PartialEq, Eq)]
pub enum Data {
    Dict(HashMap<Value, Box<Data>>),
    List(Vec<Data>),
    Set(HashSet<Value>),
    Value(Value),
}

pub enum WireValue {
    ByRef(Symbol),
    Data(Data),
    None
}
