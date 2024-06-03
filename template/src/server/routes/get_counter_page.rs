use actix_web::{
    get,
    http::header::{CacheControl, CacheDirective, ETAG},
    web, HttpRequest, HttpResponse,
};
use jetpack::http::{create_etag, get_is_etag_not_modified};
use mime::TEXT_HTML;

use crate::{structs::AppState, views};

#[get("/")]
pub async fn handle(req: HttpRequest, state: web::Data<AppState>) -> HttpResponse {
    let items = vec![
        //
        views::heading("Counter"),
        views::counter(&state.hashmap, "a"),
        views::counter(&state.hashmap, "b"),
        views::counter(&state.hashmap, "c"),
    ];

    let html = views::doc(&state.hashmap, "Counter", items);
    let buffer = html.into_bytes();

    let headers = req.headers();
    let etag = create_etag(&buffer);

    let mut builder = match get_is_etag_not_modified(headers, &etag) {
        true => HttpResponse::NotModified(),
        false => HttpResponse::Ok(),
    };

    builder.content_type(TEXT_HTML);
    builder.insert_header((ETAG, etag));
    builder.insert_header(CacheControl(vec![
        CacheDirective::Public,
        CacheDirective::MaxAge(0),
        CacheDirective::MustRevalidate,
    ]));

    return builder.body(buffer);
}
