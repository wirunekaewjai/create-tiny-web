use std::fs::read;

use actix_web::{get, HttpRequest, HttpResponse};

use crate::functions::{create_etag_response, get_file_mime};

#[get("/{filename:.*}")]
pub async fn handle(req: HttpRequest) -> HttpResponse {
    let file_path = format!("{}{}", "public", req.path());

    let Ok(buffer) = read(&file_path) else {
        return HttpResponse::NotFound().finish();
    };

    let mime = get_file_mime(&file_path);

    return create_etag_response(&req, mime, buffer);
}
