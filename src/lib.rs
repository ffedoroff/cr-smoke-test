//! Minimal crate so the Rust plugin (cargo metadata) has something to analyze.

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn greet(name: &str) -> String {
    format!("hello {name}")
}

pub fn classify(n: i32) -> &'static str {
    if n < 0 {
        "neg"
    } else if n == 0 {
        "zero"
    } else if n > 100 {
        "big"
    } else {
        "pos"
    }
}
