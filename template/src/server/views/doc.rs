use html_to_string_macro::html;
use serde_json::Value;

use crate::functions::get_asset_path;

pub fn doc(map: &Value, title: &str, children: Vec<String>) -> String {
    return html!(
        <>
            {"<!DOCTYPE html>"}
            <html lang="en">
                <head>
                    <meta charset="UTF-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1.0" />

                    <link rel="icon" sizes="any" type="image/x-icon" href="/favicon.ico" />
                    <link rel="stylesheet" href={get_asset_path(map, "/assets/style.css")} />

                    <title>{title}</title>

                    <script defer type="module" src={get_asset_path(map, "/assets/app.js")}></script>
                </head>
                <body>
                    {children.join("")}
                </body>
            </html>
        </>
    );
}
