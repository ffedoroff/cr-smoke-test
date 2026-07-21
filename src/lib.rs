//! Minimal crate so the Rust plugin (cargo metadata) has something to analyze.

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn greet(name: &str) -> String {
    format!("hello {name}")
}

/// E2E final check: full pipeline on the shipped state (v1 moved, fix deployed).
pub fn smoke_e2e_final() -> u32 {
    42
}

/// Second push: the sticky comment must be UPDATED in place, not duplicated.
pub fn smoke_e2e_second_push() -> u32 {
    43
}
