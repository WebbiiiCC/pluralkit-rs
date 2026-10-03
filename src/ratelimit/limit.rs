use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use tokio::time::sleep;
use crate::ratelimit::scope::RateLimitScope;

pub struct RateLimitInfo {
    pub remaining: u32,
    pub reset_time: u128,
}

pub struct RateLimiter {
    scopes: Arc<RwLock<HashMap<RateLimitScope, RateLimitInfo>>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            scopes: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn update(&self, scope: RateLimitScope, info: RateLimitInfo) {
        let mut scopes = self.scopes.write().await;
        scopes.insert(scope, info);
    }

    pub async fn check(&self, scope: RateLimitScope) -> Option<u64> {
        let scopes = self.scopes.read().await;
        if let Some(info) = scopes.get(&scope) {
            let now = current_time_ms();

            if info.remaining == 0 && now < info.reset_time {
                return Some((info.reset_time - now) as u64);
            }
        }
        None
    }

    pub async fn wait_if_required(&self, scope: RateLimitScope) {
        if let Some(wait_time_ms) = self.check(scope).await {
            sleep(Duration::from_millis(wait_time_ms)).await;
        }
    }
}

fn current_time_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}