use regex::Regex;
use serde_json::Value;

pub fn get_is_same_asset_hash(map: &Value, src: &str, src_hash: &Option<String>) -> bool {
    match src_hash {
        Some(value) => {
            let hash = map[src].as_str().unwrap_or_default();
            return value.len() > 0 && value.eq(hash);
        }
        None => {
            let regex = Regex::new(r"^\/assets\/chunk\-[a-z0-9]{16,32}\.[A-Za-z0-9]+$")
                .expect("invalid regex");

            return regex.is_match(src);
        }
    }
}
