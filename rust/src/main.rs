#![allow(dead_code)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate random_number;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::{web::Data, App, HttpServer};

mod errors;
mod mocks;
mod routes;
mod search;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix=info");
    env_logger::init();

    HttpServer::new(move || {
        let cors = Cors::default().allow_any_origin();

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            // .data(mocks::api::PartSearchAPI {})
            .app_data(Data::new(mocks::api::PartSearchAPI {}))
            .service(routes::healthcheck)
            .service(web::scope("").route("/search", web::get().to(routes::search::search_by_mpn)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
