use actix_web::{get, web, HttpRequest, HttpResponse};
use mime::TEXT_HTML;
use serde::Deserialize;

use crate::views;

#[derive(Deserialize)]
struct CounterQuery {
    count: Option<i32>,
}

#[get("/")]
pub async fn handle(req: HttpRequest, query: web::Query<CounterQuery>) -> HttpResponse {
    let count = query.count.unwrap_or(0);
    let items = vec![
        //
        views::heading("Counter"),
        views::counter(count),
    ];

    let html = views::doc("Counter", items);
    let buffer = html.into_bytes();

    let mut builder = HttpResponse::Ok();

    builder.content_type(TEXT_HTML);

    jetpack::bind_etag_header(&mut builder, &req, &buffer);

    return builder.body(buffer);
}
