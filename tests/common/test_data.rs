use std::sync::atomic::{AtomicU64, Ordering};

static KEY_COUNTER: AtomicU64 = AtomicU64::new(0);

pub fn unique_key(prefix: &str) -> String {
    let counter = KEY_COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("{}_{}", prefix, counter)
}

#[allow(dead_code)]
pub fn test_value(size: usize) -> String {
    "x".repeat(size)
}
