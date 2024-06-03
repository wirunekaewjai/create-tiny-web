pub mod functions;
pub mod routes;
pub mod structs;
pub mod views;

use actix_web::{web, App, HttpServer};
use functions::get_asset_hashmap;
use structs::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let hostname = "0.0.0.0";
    let port = 8080;

    println!();
    println!("Running at http://0.0.0.0:{}", port);
    println!();

    HttpServer::new(move || {
        let hashmap = get_asset_hashmap();
        let app_state = AppState { hashmap };

        App::new()
            .app_data(web::Data::new(app_state))
            .service(routes::get_counter_page::handle)
            .service(routes::get_asset::handle)
    })
    .bind((hostname, port))?
    .run()
    .await
}
