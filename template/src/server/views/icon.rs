use html_to_string_macro::html;
use serde_json::Value;

use crate::functions::get_asset_path;

pub fn icon(map: &Value, name: &str) -> String {
    let path = get_asset_path(map, &format!("/icons/{name}.svg"));

    return html!(
        <svg
            name={name}
            hx-get={&path}
            hx-trigger="load"
            hx-swap="outerHTML"
            viewBox="0 0 1 1"
        />
    );
}
