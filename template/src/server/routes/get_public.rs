use std::fs::read;

use actix_web::{get, HttpRequest, HttpResponse};

#[get("/{filename:.*}")]
pub async fn handle(req: HttpRequest) -> HttpResponse {
    let file_path = format!("{}{}", "public", req.path());

    let Ok(buffer) = read(&file_path) else {
        return HttpResponse::NotFound().finish();
    };

    let mime = rs::get_file_mime(&file_path);

    return rs::create_etag_response(&req, mime, buffer);
}
