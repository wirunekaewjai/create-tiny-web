pub mod routes;
pub mod views;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let hostname = "0.0.0.0";
    let port = 8080;

    println!();
    println!("Running at http://0.0.0.0:{}", port);
    println!();

    HttpServer::new(move || {
        App::new()
            .service(routes::get_asset::handle)
            .service(routes::get_counter_page::handle)
            .service(routes::get_public::handle)
    })
    .bind((hostname, port))?
    .run()
    .await
}
