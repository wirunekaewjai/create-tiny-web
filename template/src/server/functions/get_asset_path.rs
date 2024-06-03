use serde_json::Value;

pub fn get_asset_path(map: &Value, src: &str) -> String {
    let hash = map[src].as_str().unwrap_or_default();

    if hash.len() > 0 {
        return format!("{src}?hash={hash}");
    }

    src.to_string()
}
