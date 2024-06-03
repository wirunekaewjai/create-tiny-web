use serde_json::Value;

pub fn get_is_same_asset_hash(map: &Value, src: &str, src_hash: &Option<String>) -> bool {
    match src_hash {
        Some(value) => {
            let hash = map[src].as_str().unwrap_or_default();
            return value.len() > 0 && value.eq(hash);
        }
        None => false,
    }
}
