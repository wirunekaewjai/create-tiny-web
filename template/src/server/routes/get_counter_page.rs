use actix_web::{get, HttpRequest, HttpResponse};
use mime::TEXT_HTML;

use crate::views;

#[get("/")]
pub async fn handle(req: HttpRequest) -> HttpResponse {
    let items = vec![
        //
        views::heading("Counter"),
        views::counter("a"),
        views::counter("b"),
        views::counter("c"),
    ];

    let html = views::doc("Counter", items);
    let buffer = html.into_bytes();

    let mut builder = HttpResponse::Ok();

    builder.content_type(TEXT_HTML);

    jetpack::bind_etag_header(&mut builder, &req, &buffer);

    return builder.body(buffer);
}
