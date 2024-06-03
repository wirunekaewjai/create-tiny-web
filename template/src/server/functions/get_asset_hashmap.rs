use std::fs::read_to_string;

use serde_json::{from_str, Value};

pub fn get_asset_hashmap() -> Value {
    let text = read_to_string(".cache/hashmap.json").expect("failed to read hashmap.json");
    let json = from_str(&text).expect("invalid hashmap.json");

    return json;
}
