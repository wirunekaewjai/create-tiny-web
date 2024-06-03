use std::fs::read;

use actix_web::{
    get,
    http::header::{CacheControl, CacheDirective, ETAG},
    HttpRequest, HttpResponse,
};
use jetpack::http::{create_etag, get_file_mime, get_is_etag_not_modified};

#[get("/assets/{filename:.*}")]
pub async fn handle(req: HttpRequest) -> HttpResponse {
    let Some(file_path) = req.path().get(1..) else {
        return HttpResponse::NotFound().finish();
    };

    let Ok(buffer) = read(&file_path) else {
        return HttpResponse::NotFound().finish();
    };

    let headers = req.headers();
    let mime = get_file_mime(&file_path);
    let etag = create_etag(&buffer);

    let mut builder = match get_is_etag_not_modified(headers, &etag) {
        true => HttpResponse::NotModified(),
        false => HttpResponse::Ok(),
    };

    builder.content_type(mime);
    builder.insert_header((ETAG, etag));
    builder.insert_header(CacheControl(vec![
        CacheDirective::Public,
        CacheDirective::MaxAge(0),
        CacheDirective::MustRevalidate,
    ]));

    return builder.body(buffer);
}
