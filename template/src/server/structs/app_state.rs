use serde_json::Value;

#[derive(Clone)]
pub struct AppState {
    pub hashmap: Value,
}
