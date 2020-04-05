mod models;

use actix_web::{ HttpServer, App, web, Responder };
use std::io;

use crate::models::Status;


async fn status() -> impl Responder {
    web::HttpResponse::Ok()
        .json(Status { status: "UP".to_string() })
}

#[actix_rt::main]
async fn main() -> io::Result<()> {

    println!("Brought up mighty server on 127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(status))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
