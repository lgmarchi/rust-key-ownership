use std::collections::HashSet;

use tokio::sync::Mutex;

#[derive(Default)]
pub struct AppState {
    pub seen_nonces: Mutex<HashSet<String>>,
}
