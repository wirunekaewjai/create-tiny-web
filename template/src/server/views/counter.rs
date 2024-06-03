use html_to_string_macro::html;
use serde_json::Value;

use super::icon;

pub fn counter(map: &Value, name: &str) -> String {
    return html!(
        <div
            class="p-2 flex flex-row items-center"
            id={name}
            hx-swap="outerHTML"
            hx-swap-oob="true"
        >
            <button
                class="w-8 h-8 bg-red-600 text-white rounded-md shadow-md fill-current flex items-center justify-center p-2"
                hx-get={format!("/@count?name={name}&value=-1")}
            >
                {icon(map, "fa-solid-minus")}
            </button>
            <div
                class="flex items-center px-4 h-8 mx-2 border rounded-md"
            >
                {0}
            </div>
            <button
                class="w-8 h-8 bg-blue-600 text-white rounded-md shadow-md fill-current flex items-center justify-center p-2"
                hx-get={format!("/@count?name={name}&value=1")}
            >
                {icon(map, "fa-solid-plus")}
            </button>
        </div>
    );
}
