use std::fs::read;

use actix_web::{
    get,
    http::{
        header::{CacheControl, CacheDirective, ETAG},
        StatusCode,
    },
    web, HttpRequest, HttpResponse,
};
use jetpack::functions::http::{create_etag, get_file_mime, get_is_etag_not_modified};
use serde::Deserialize;

use crate::{functions::get_is_same_asset_hash, structs::AppState};

#[derive(Deserialize)]
pub struct Query {
    pub hash: Option<String>,
}

#[get("/{filename:.*}")]
pub async fn handle(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<String>,
    query: web::Query<Query>,
) -> HttpResponse {
    let req_path = req.path();
    let mut file_path = path.into_inner();

    if !file_path.starts_with("assets/") {
        file_path = format!("public/{file_path}");
    }

    let Ok(buffer) = read(&file_path) else {
        return HttpResponse::NotFound().finish();
    };

    let mime = get_file_mime(&file_path);
    let mut builder = HttpResponse::Ok();

    builder.content_type(mime);

    if get_is_same_asset_hash(&state.hashmap, req_path, &query.hash) {
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
