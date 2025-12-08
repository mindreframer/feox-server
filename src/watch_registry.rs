use dashmap::DashMap;
use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicU64, Ordering};

static GLOBAL_VERSION: AtomicU64 = AtomicU64::new(1);
static KEY_VERSIONS: Lazy<DashMap<Vec<u8>, u64>> = Lazy::new(DashMap::new);

pub struct WatchRegistry;

impl WatchRegistry {
    pub fn notify_key_modified(key: &[u8]) {
        let version = GLOBAL_VERSION.fetch_add(1, Ordering::SeqCst);
        KEY_VERSIONS.insert(key.to_vec(), version);
    }

    pub fn get_key_version(key: &[u8]) -> u64 {
        KEY_VERSIONS.get(key).map(|v| *v).unwrap_or(0)
    }

    pub fn check_keys_unchanged(watched_versions: &[(Vec<u8>, u64)]) -> bool {
        for (key, watched_version) in watched_versions {
            let current_version = Self::get_key_version(key);
            if current_version != *watched_version {
                return false;
            }
        }
        true
    }
}
