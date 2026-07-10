//! Minimal crate so the Rust plugin (cargo metadata) has something to analyze.

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn greet(name: &str) -> String {
    format!("hello {name}")
}

pub fn mul(a: i32, b: i32) -> i32 {
    a * b
}
