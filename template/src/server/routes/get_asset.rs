use std::fs::read;

use actix_web::{get, HttpRequest, HttpResponse};

use crate::functions;

#[get("/assets/{filename:.*}")]
pub async fn handle(req: HttpRequest) -> HttpResponse {
    let Some(file_path) = req.path().get(1..) else {
        return HttpResponse::NotFound().finish();
    };

    let Ok(buffer) = read(&file_path) else {
        return HttpResponse::NotFound().finish();
    };

    let mime = functions::get_file_mime(&file_path);

    return functions::create_etag_response(&req, mime, buffer);
}
