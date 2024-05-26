use html_to_string_macro::html;
use serde_json::json;

pub fn counter(count: i32) -> String {
    let dec = count - 1;
    let inc = count + 1;
    let vals = json!({ "count": dec }).to_string().replace('"', "&quot;");

    return html!(
        <div
            class="p-2 flex flex-row items-center"
            hx-target="this"
            hx-swap="outerHTML"
        >
            <button
                class="w-8 h-8 bg-red-600 text-white rounded-md shadow-md"
                hx-get="/@counter"
                hx-vals={vals}
                hx-trigger="click"
                hx-replace-url={format!("/?count={}", dec)}
            >
                {"-"}
            </button>
            <div class="flex items-center px-4 h-8 mx-2 border rounded-md">
                {count}
            </div>
            <button
                class="w-8 h-8 bg-blue-600 text-white rounded-md shadow-md"
                hx-get={format!("/@counter?count={}", inc)}
                hx-trigger="click"
                hx-replace-url={format!("/?count={}", inc)}
            >
                {"+"}
            </button>
        </div>
    );
}
