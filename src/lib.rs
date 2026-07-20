//! Minimal crate so the Rust plugin (cargo metadata) has something to analyze.

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn greet(name: &str) -> String {
    format!("hello {name}")
}

/// Smoke: exercise the App-comment pipeline (same-repo PR).
pub fn smoke_app_comment_marker() -> &'static str {
    "app-comment-same-repo"
}
