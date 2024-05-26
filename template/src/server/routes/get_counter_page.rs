use actix_web::{get, web, HttpRequest, HttpResponse};
use mime::TEXT_HTML;
use serde::Deserialize;

use crate::{functions::create_etag_response, views};

#[derive(Deserialize)]
struct CounterQuery {
    count: Option<i32>,
}

#[get("/")]
pub async fn handle(req: HttpRequest, query: web::Query<CounterQuery>) -> HttpResponse {
    let count = query.count.unwrap_or(0);
    let items = vec![
        views::heading("Counter"),
        views::counter(count),
    ];

    let html = views::doc("Counter", items);

    return create_etag_response(&req, TEXT_HTML, html.into_bytes());
}
