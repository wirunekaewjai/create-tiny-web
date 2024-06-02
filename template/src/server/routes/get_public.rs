use std::fs::read;

use actix_web::{
    get,
    http::header::{CacheControl, CacheDirective},
    web, HttpRequest, HttpResponse,
};
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

    let mime = jetpack::get_file_mime(&file_path);
    let mut builder = HttpResponse::Ok();

    builder.content_type(mime);

    match &query.v {
        Some(_) => {
            builder.insert_header(CacheControl(vec![
                CacheDirective::Public,
                CacheDirective::MaxAge(31_536_000),
            ]));
        }

        None => jetpack::bind_etag_header(&mut builder, &req, &buffer),
    }

    return builder.body(buffer);
}
