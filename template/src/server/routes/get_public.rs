use std::fs::read;

use actix_web::{
    get,
    http::{
        header::{CacheControl, CacheDirective, ETAG},
        StatusCode,
    },
    web, HttpRequest, HttpResponse,
};
use jetpack::http::{create_etag, get_file_mime, get_is_etag_not_modified};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Query {
    pub v: Option<String>,
}

#[get("/{filename:.*}")]
pub async fn handle(req: HttpRequest, query: web::Query<Query>) -> HttpResponse {
    let file_path = format!("{}{}", "public", req.path());

    let Ok(buffer) = read(&file_path) else {
        return HttpResponse::NotFound().finish();
    };

    let mime = get_file_mime(&file_path);
    let mut builder = HttpResponse::Ok();

    builder.content_type(mime);

    if query.v.is_some() {
        builder.insert_header(CacheControl(vec![
            CacheDirective::Public,
            CacheDirective::MaxAge(31_536_000),
        ]));
    } else {
        let headers = req.headers();
        let etag = create_etag(&buffer);

        if get_is_etag_not_modified(headers, &etag) {
            builder.status(StatusCode::NOT_MODIFIED);
        }

        builder.insert_header((ETAG, etag));
        builder.insert_header(CacheControl(vec![
            CacheDirective::Public,
            CacheDirective::MaxAge(0),
            CacheDirective::MustRevalidate,
        ]));
    }

    return builder.body(buffer);
}
